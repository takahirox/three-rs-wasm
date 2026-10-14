// Transpiles three r186 ES modules into Rust over tsl/src/rt ( the JavaScript
// object model ): node tools/tsl/transpile.mjs <path relative to three/src> …
// Each module becomes tsl/src/three/<snake path>.rs with an `init()` that
// evaluates the module body. Semantics follow the source: bindings, closures
// over shared cells, classes as constructor + prototype, getters, generators
// ( eager ), try/catch, destructuring, spread, and V8's TypeError texts built
// from the callee's source text.
import * as acorn from 'acorn';
import fs from 'node:fs';
import path from 'node:path';

const ROOT = new URL( '../../', import.meta.url ).pathname;
const SRC = ROOT + '.cache/three-r186/src/';
const OUT = ROOT + 'tsl/src/three/';
const OVERRIDES = ROOT + 'tools/tsl/overrides/';
// Bare specifiers the sources import.
const BARE = { 'three/webgpu': 'Three.WebGPU.js', 'three': 'Three.WebGPU.js' };

// The source of a module: an override replaces it, and the original stays
// importable as `<name>.original.js`.
export function sourceOf( jsPath ) {
	if ( jsPath.endsWith( '.original.js' ) ) return fs.readFileSync( SRC + jsPath.replace( '.original.js', '.js' ), 'utf8' );
	if ( fs.existsSync( OVERRIDES + jsPath ) ) return fs.readFileSync( OVERRIDES + jsPath, 'utf8' );
	return fs.readFileSync( SRC + jsPath, 'utf8' );
}

const KEYWORDS = new Set( [ 'core', 'loop', 'type', 'match', 'mod', 'fn', 'impl', 'use', 'ref', 'self', 'super', 'move', 'box', 'where', 'const', 'static', 'struct', 'enum', 'trait', 'crate', 'extern', 'in', 'as', 'break', 'continue', 'else', 'if', 'for', 'while', 'return', 'true', 'false', 'let', 'mut', 'pub', 'unsafe', 'dyn', 'async', 'await', 'abstract', 'final', 'override', 'virtual', 'yield', 'try', 'macro', 'priv', 'typeof', 'unsized', 'do', 'become' ] );

export function snake( name ) {
	let s = name.replace( /([A-Z]+)([A-Z][a-z])/g, '$1_$2' ).replace( /([a-z0-9])([A-Z])/g, '$1_$2' );
	s = s.replace( /[-.]/g, '_' ).toLowerCase().replace( /_+/g, '_' );
	return KEYWORDS.has( s ) ? s + '_' : s;
}

// A file whose snake name is also a sibling directory's ( nodes/TSL.js beside nodes/tsl/ ) gets `_mod`.
export function rustPath( js ) {
	const parts = js.slice( 0, - 3 ).split( '/' );
	const out = parts.map( snake );
	if ( js.endsWith( '.original.js' ) ) return out;
	const dir = SRC + parts.slice( 0, - 1 ).join( '/' );
	const last = out[ out.length - 1 ];
	if ( fs.existsSync( dir ) && fs.readdirSync( dir ).some( f => fs.statSync( dir + '/' + f ).isDirectory() && snake( f ) === last ) ) out[ out.length - 1 ] = last + '_mod';
	return out;
}

const rs = ( text ) => JSON.stringify( text ); // a Rust string literal ( JSON escapes are valid Rust escapes )

function rustStr( text ) {
	let out = '"';
	for ( const ch of text ) {
		const c = ch.codePointAt( 0 );
		if ( ch === '"' ) out += '\\"';
		else if ( ch === '\\' ) out += '\\\\';
		else if ( ch === '\n' ) out += '\\n';
		else if ( ch === '\r' ) out += '\\r';
		else if ( ch === '\t' ) out += '\\t';
		else if ( c < 0x20 || c === 0x7f ) out += '\\u{' + c.toString( 16 ) + '}';
		else out += ch;
	}
	return out + '"';
}

function numLit( v ) {
	if ( Number.isNaN( v ) ) return 'f64::NAN';
	if ( v === Infinity ) return 'f64::INFINITY';
	if ( v === - Infinity ) return 'f64::NEG_INFINITY';
	let t = String( v );
	if ( ! /[.e]/.test( t ) ) t += '.0';
	if ( t.startsWith( '.' ) ) t = '0' + t;
	return t.replace( 'e+', 'e' );
}

// ------------------------------------------------------------------ scope analysis

class Scope {

	constructor( parent, fn, kind ) {
		this.parent = parent;
		this.fn = fn; // the function ( or 'module' ) owning the scope
		this.kind = kind;
		this.names = new Map(); // name -> binding
	}

	lookup( name ) {
		for ( let s = this; s; s = s.parent ) {
			const b = s.names.get( name );
			if ( b ) return b;
		}
		return null;
	}

}

let uniq = 0;

function declare( scope, name, kind ) {
	const existing = scope.names.get( name );
	if ( existing ) return existing;
	const b = { name, kind, scope, fn: scope.fn, captured: false, id: uniq ++, rust: null };
	scope.names.set( name, b );
	return b;
}

function patternNames( p, out = [] ) {
	if ( ! p ) return out;
	switch ( p.type ) {
		case 'Identifier': out.push( p.name ); break;
		case 'ObjectPattern': for ( const prop of p.properties ) patternNames( prop.type === 'RestElement' ? prop.argument : prop.value, out ); break;
		case 'ArrayPattern': for ( const el of p.elements ) patternNames( el, out ); break;
		case 'RestElement': patternNames( p.argument, out ); break;
		case 'AssignmentPattern': patternNames( p.left, out ); break;
	}
	return out;
}

// Assign scopes to every node and record references: scopeOf( node ) gives the
// scope a node's identifiers resolve in.
function analyze( program ) {
	const scopes = new Map();
	const moduleScope = new Scope( null, 'module', 'module' );
	scopes.set( program, moduleScope );
	const refs = []; // [ identifier node, scope ]

	function hoistVars( body, fnScope ) {
		// `var` and function declarations of a function body.
		const visit = ( node, inNested ) => {
			if ( ! node || typeof node !== 'object' ) return;
			if ( Array.isArray( node ) ) {
				for ( const n of node ) visit( n, inNested );
				return;
			}
			if ( node.type === 'VariableDeclaration' && node.kind === 'var' ) {
				for ( const d of node.declarations ) for ( const n of patternNames( d.id ) ) declare( fnScope, n, 'var' );
			}
			if ( node.type === 'FunctionDeclaration' || node.type === 'FunctionExpression' || node.type === 'ArrowFunctionExpression' || node.type === 'ClassDeclaration' || node.type === 'ClassExpression' ) return;
			for ( const k of Object.keys( node ) ) {
				if ( k === 'type' || k === 'start' || k === 'end' ) continue;
				const v = node[ k ];
				if ( v && typeof v === 'object' ) visit( v, inNested );
			}
		};
		visit( body, false );
	}

	function blockDecls( stmts, scope ) {
		for ( const st of stmts ) {
			if ( st.type === 'VariableDeclaration' && st.kind !== 'var' ) {
				for ( const d of st.declarations ) for ( const n of patternNames( d.id ) ) declare( scope, n, st.kind );
			} else if ( st.type === 'FunctionDeclaration' ) {
				declare( scope, st.id.name, 'function' );
			} else if ( st.type === 'ClassDeclaration' ) {
				declare( scope, st.id.name, 'class' );
			} else if ( st.type === 'ImportDeclaration' ) {
				for ( const sp of st.specifiers ) declare( scope, sp.local.name, 'import' ).import = { source: st.source.value, imported: sp.type === 'ImportDefaultSpecifier' ? 'default' : sp.type === 'ImportNamespaceSpecifier' ? '*' : sp.imported.name };
			} else if ( st.type === 'ExportNamedDeclaration' && st.declaration ) {
				blockDecls( [ st.declaration ], scope );
			} else if ( st.type === 'ExportDefaultDeclaration' && st.declaration && ( st.declaration.type === 'FunctionDeclaration' || st.declaration.type === 'ClassDeclaration' ) && st.declaration.id ) {
				blockDecls( [ st.declaration ], scope );
			}
		}
	}

	function walk( node, scope ) {
		if ( ! node || typeof node !== 'object' ) return;
		if ( Array.isArray( node ) ) {
			for ( const n of node ) walk( n, scope );
			return;
		}
		switch ( node.type ) {
			case 'Program':
				hoistVars( node.body, scope );
				blockDecls( node.body, scope );
				for ( const st of node.body ) walk( st, scope );
				return;
			case 'FunctionDeclaration':
			case 'FunctionExpression':
			case 'ArrowFunctionExpression': {
				const fnScope = new Scope( scope, node, 'function' );
				scopes.set( node, fnScope );
				if ( node.type === 'FunctionDeclaration' && node.id ) scopes.set( node.id, scope );
				if ( node.type === 'FunctionExpression' && node.id ) {
					declare( fnScope, node.id.name, 'fname' );
					scopes.set( node.id, fnScope );
				}
				for ( const p of node.params ) for ( const n of patternNames( p ) ) declare( fnScope, n, 'param' );
				if ( node.body.type === 'BlockStatement' ) {
					// Parameter defaults see the parameters and the outer scope, not the body's declarations.
					hoistVars( node.body.body, fnScope );
					const bodyScope = new Scope( fnScope, node, 'block' );
					blockDecls( node.body.body, bodyScope );
					scopes.set( node.body, bodyScope );
					for ( const p of node.params ) walkPattern( p, fnScope );
					for ( const st of node.body.body ) walk( st, bodyScope );
				} else {
					for ( const p of node.params ) walkPattern( p, fnScope );
					walk( node.body, fnScope );
				}
				return;
			}
			case 'ClassDeclaration':
			case 'ClassExpression': {
				const cScope = new Scope( scope, scope.fn, 'class' );
				scopes.set( node, cScope );
				if ( node.id && node.type === 'ClassExpression' ) {
					declare( cScope, node.id.name, 'class' );
					scopes.set( node.id, cScope );
				} else if ( node.id ) {
					scopes.set( node.id, scope );
				}
				walk( node.superClass, scope );
				for ( const m of node.body.body ) {
					if ( m.computed ) walk( m.key, cScope );
					if ( m.type === 'StaticBlock' ) {
						const fnScope = new Scope( cScope, m, 'function' );
						scopes.set( m, fnScope );
						hoistVars( m.body, fnScope );
						blockDecls( m.body, fnScope );
						for ( const st of m.body ) walk( st, fnScope );
					} else if ( m.type === 'PropertyDefinition' ) {
						const fnScope = new Scope( cScope, m, 'function' );
						scopes.set( m, fnScope );
						walk( m.value, fnScope );
					} else {
						walk( m.value, cScope );
					}
				}
				return;
			}
			case 'BlockStatement': {
				const b = new Scope( scope, scope.fn, 'block' );
				scopes.set( node, b );
				blockDecls( node.body, b );
				for ( const st of node.body ) walk( st, b );
				return;
			}
			case 'ForStatement':
			case 'ForOfStatement':
			case 'ForInStatement': {
				const b = new Scope( scope, scope.fn, 'block' );
				scopes.set( node, b );
				const init = node.init || node.left;
				if ( init && init.type === 'VariableDeclaration' && init.kind !== 'var' ) {
					for ( const d of init.declarations ) for ( const n of patternNames( d.id ) ) declare( b, n, init.kind );
				}
				for ( const k of [ 'init', 'test', 'update', 'left', 'right', 'body' ] ) {
					if ( k === 'left' && node.left && node.left.type === 'VariableDeclaration' ) {
						for ( const d of node.left.declarations ) walkPattern( d.id, b );
						continue;
					}
					walk( node[ k ], b );
				}
				return;
			}
			case 'SwitchStatement': {
				walk( node.discriminant, scope );
				const b = new Scope( scope, scope.fn, 'block' );
				scopes.set( node, b );
				for ( const c of node.cases ) blockDecls( c.consequent, b );
				for ( const c of node.cases ) {
					walk( c.test, b );
					for ( const st of c.consequent ) walk( st, b );
				}
				return;
			}
			case 'CatchClause': {
				const b = new Scope( scope, scope.fn, 'block' );
				scopes.set( node, b );
				if ( node.param ) {
					for ( const n of patternNames( node.param ) ) declare( b, n, 'let' );
					walkPattern( node.param, b );
				}
				walk( node.body, b );
				return;
			}
			case 'VariableDeclarator':
				walkPattern( node.id, scope );
				walk( node.init, scope );
				return;
			case 'Identifier':
				refs.push( [ node, scope ] );
				scopes.set( node, scope );
				return;
			case 'MemberExpression':
				walk( node.object, scope );
				if ( node.computed ) walk( node.property, scope );
				return;
			case 'Property':
				if ( node.computed ) walk( node.key, scope );
				walk( node.value, scope );
				return;
			case 'MethodDefinition':
				walk( node.value, scope );
				return;
			case 'LabeledStatement':
				walk( node.body, scope );
				return;
			case 'BreakStatement':
			case 'ContinueStatement':
				return;
			case 'ImportDeclaration':
				return;
			case 'ExportNamedDeclaration':
				if ( node.declaration ) walk( node.declaration, scope );
				else for ( const sp of node.specifiers ) if ( ! node.source ) walk( sp.local, scope );
				return;
			case 'ExportDefaultDeclaration':
				walk( node.declaration, scope );
				return;
			case 'ExportAllDeclaration':
				return;
			default:
				for ( const k of Object.keys( node ) ) {
					if ( k === 'type' || k === 'start' || k === 'end' ) continue;
					const v = node[ k ];
					if ( v && typeof v === 'object' ) walk( v, scope );
				}
		}
	}

	function walkPattern( p, scope ) {
		if ( ! p ) return;
		switch ( p.type ) {
			case 'Identifier': scopes.set( p, scope ); return;
			case 'ObjectPattern':
				for ( const prop of p.properties ) {
					if ( prop.type === 'RestElement' ) walkPattern( prop.argument, scope );
					else {
						if ( prop.computed ) walk( prop.key, scope );
						walkPattern( prop.value, scope );
					}
				}
				return;
			case 'ArrayPattern': for ( const el of p.elements ) walkPattern( el, scope ); return;
			case 'RestElement': walkPattern( p.argument, scope ); return;
			case 'AssignmentPattern': walkPattern( p.left, scope ); walk( p.right, scope ); return;
			default: walk( p, scope );
		}
	}

	walk( program, moduleScope );
	// Captures: references from another function than the binding's.
	for ( const [ id, scope ] of refs ) {
		const b = scope.lookup( id.name );
		if ( b && b.scope !== moduleScope && b.fn !== scope.fn ) b.captured = true;
	}
	return { scopes, moduleScope };
}

// ------------------------------------------------------------------ code generation

class Gen {

	constructor( jsPath, source ) {
		this.jsPath = jsPath;
		this.source = source;
		this.ast = acorn.parse( source, { ecmaVersion: 2024, sourceType: 'module' } );
		const { scopes, moduleScope } = analyze( this.ast );
		this.scopes = scopes;
		this.moduleScope = moduleScope;
		this.slots = 0;
		this.exportsOf = new Map(); // binding name -> exported names
		this.loops = []; // { label, cont, closureDepth }
		this.closureDepth = 0; // nested try closures inside the current function
		this.fnStack = [];
		this.tmp = 0;
		this.collectExports();
	}

	text( node ) {
		return this.source.slice( node.start, node.end ).replace( /\s+/g, ' ' );
	}

	t() {
		return `t${ this.tmp ++ }`;
	}

	collectExports() {
		for ( const st of this.ast.body ) {
			if ( st.type === 'ExportNamedDeclaration' && ! st.source ) {
				if ( st.declaration ) {
					const names = st.declaration.type === 'VariableDeclaration' ? st.declaration.declarations.flatMap( d => patternNames( d.id ) ) : [ st.declaration.id.name ];
					for ( const n of names ) this.addExport( n, n );
				} else {
					for ( const sp of st.specifiers ) this.addExport( sp.local.name, sp.exported.name ?? sp.exported.value );
				}
			} else if ( st.type === 'ExportDefaultDeclaration' && st.declaration.id && ( st.declaration.type === 'ClassDeclaration' || st.declaration.type === 'FunctionDeclaration' ) ) {
				this.addExport( st.declaration.id.name, 'default' );
			}
		}
		// Module bindings: exported ones live in the registry, the rest in slots.
		for ( const b of this.moduleScope.names.values() ) {
			if ( b.kind === 'import' ) continue;
			const ex = this.exportsOf.get( b.name );
			if ( ex ) b.exportName = ex[ 0 ];
			else b.slot = this.slots ++;
		}
	}

	addExport( local, exported ) {
		if ( ! this.exportsOf.has( local ) ) this.exportsOf.set( local, [] );
		this.exportsOf.get( local ).push( exported );
	}

	scopeOf( node ) {
		return this.scopes.get( node );
	}

	resolveModule( spec ) {
		if ( BARE[ spec ] ) return BARE[ spec ];
		return path.normalize( path.join( path.dirname( this.jsPath ), spec ) );
	}

	// The Rust name of a local binding.
	local( b ) {
		if ( ! b.rust ) b.rust = 'v_' + b.name.replace( /\$/g, '_d' ) + ( b.shadowed ? '_' + b.id : '' );
		return b.rust;
	}

	// ---------------------------------------------------------------- bindings

	read( name, node ) {
		const scope = this.scopeOf( node );
		const b = scope ? scope.lookup( name ) : this.moduleScope.lookup( name );
		if ( ! b ) {
			if ( name === 'undefined' ) return 'Val::Undef';
			if ( name === 'arguments' ) return 'array(args.to_vec())';
			return `glob(${ rustStr( name ) })`;
		}
		if ( b.kind === 'import' ) {
			if ( b.import.imported === '*' ) return `module_namespace(${ rustStr( this.resolveModule( b.import.source ) ) })`;
			return `imp(${ rustStr( this.resolveModule( b.import.source ) ) }, ${ rustStr( b.import.imported ) })`;
		}
		if ( b.scope === this.moduleScope ) {
			if ( b.exportName !== undefined ) return `imp(M, ${ rustStr( b.exportName ) })`;
			return `slot(${ b.slot })`;
		}
		const r = this.local( b );
		return b.captured ? `${ r }.borrow().clone()` : `${ r }.clone()`;
	}

	// A statement writing `value` ( a Rust expression ) to a binding.
	write( name, node, value ) {
		const scope = this.scopeOf( node );
		const b = scope ? scope.lookup( name ) : null;
		if ( ! b ) return `set(&glob("globalThis"), ${ rustStr( name ) }, ${ value })?;`;
		if ( b.scope === this.moduleScope ) {
			if ( b.exportName !== undefined ) {
				return `{ let w = ${ value }; ${ this.exportsOf.get( name ).map( n => `export(M, ${ rustStr( n ) }, w.clone());` ).join( ' ' ) } }`;
			}
			return `set_slot(${ b.slot }, ${ value });`;
		}
		const r = this.local( b );
		return b.captured ? `*${ r }.borrow_mut() = ${ value };` : `${ r } = ${ value };`;
	}

	// Captured block bindings exist from the block's start, so functions
	// hoisted above their declarations can capture them.
	predeclare( scope ) {
		if ( ! scope ) return '';
		const out = [];
		for ( const b of scope.names.values() ) {
			if ( b.scope === this.moduleScope || ! b.captured ) continue;
			if ( [ 'let', 'const', 'class', 'function' ].includes( b.kind ) ) {
				b.predeclared = true;
				out.push( `let ${ this.local( b ) }: Var = var(Val::Undef);` );
			}
		}
		return out.join( ' ' );
	}

	// A `let` introducing a binding with an initial value.
	declareLocal( name, node, value ) {
		const scope = this.scopeOf( node );
		const b = scope.lookup( name );
		if ( b.scope === this.moduleScope ) return this.write( name, node, value );
		const r = this.local( b );
		if ( b.kind === 'var' && b.hoisted ) return this.write( name, node, value );
		if ( b.predeclared ) return this.write( name, node, value );
		return b.captured ? `let ${ r }: Var = var(${ value });` : `#[allow(unused_mut)] let mut ${ r }: Val = ${ value };`;
	}

	// ---------------------------------------------------------------- expressions

	expr( n ) {
		switch ( n.type ) {
			case 'Literal':
				if ( n.regex ) return `regexp(${ rustStr( n.regex.pattern ) }, ${ rustStr( n.regex.flags ) })`;
				if ( n.value === null ) return 'Val::Null';
				if ( typeof n.value === 'string' ) return `s(${ rustStr( n.value ) })`;
				if ( typeof n.value === 'number' ) return `Val::Num(${ numLit( n.value ) })`;
				if ( typeof n.value === 'boolean' ) return `Val::Bool(${ n.value })`;
				throw new Error( 'literal ' + n.raw );
			case 'Identifier':
				return this.read( n.name, n );
			case 'ThisExpression':
				return 'this_v.clone()';
			case 'TemplateLiteral': {
				const parts = [];
				for ( let i = 0; i < n.quasis.length; i ++ ) {
					if ( n.quasis[ i ].value.cooked ) parts.push( `s(${ rustStr( n.quasis[ i ].value.cooked ) })` );
					if ( i < n.expressions.length ) parts.push( this.expr( n.expressions[ i ] ) );
				}
				return `template(&[${ parts.join( ', ' ) }])?`;
			}
			case 'ArrayExpression':
				return this.arrayLit( n.elements );
			case 'ObjectExpression':
				return this.objectLit( n );
			case 'FunctionExpression':
			case 'ArrowFunctionExpression':
				return this.fnExpr( n, n.id ? n.id.name : '' );
			case 'ClassExpression':
				return this.classExpr( n );
			case 'MemberExpression':
				return this.member( n );
			case 'ChainExpression':
				return this.chain( n.expression );
			case 'CallExpression':
				return this.callExpr( n );
			case 'NewExpression': {
				const t = this.t();
				return `{ let ${ t } = ${ this.expr( n.callee ) }; let a = ${ this.args( n.arguments ) }; new_site(&${ t }, &a, ${ rustStr( this.text( n.callee ) ) })? }`;
			}
			case 'UnaryExpression':
				return this.unary( n );
			case 'UpdateExpression':
				return this.update( n );
			case 'BinaryExpression':
				return this.binary( n.operator, n.left, n.right );
			case 'LogicalExpression': {
				const t = this.t();
				const l = this.expr( n.left ), r = this.expr( n.right );
				if ( n.operator === '&&' ) return `{ let ${ t } = ${ l }; if ${ t }.truthy() { ${ r } } else { ${ t } } }`;
				if ( n.operator === '||' ) return `{ let ${ t } = ${ l }; if ${ t }.truthy() { ${ t } } else { ${ r } } }`;
				return `{ let ${ t } = ${ l }; if ${ t }.is_nullish() { ${ r } } else { ${ t } } }`;
			}
			case 'ConditionalExpression':
				return `(if (${ this.expr( n.test ) }).truthy() { ${ this.expr( n.consequent ) } } else { ${ this.expr( n.alternate ) } })`;
			case 'AssignmentExpression':
				return this.assign( n );
			case 'SequenceExpression': {
				const parts = n.expressions.map( e => this.expr( e ) );
				const last = parts.pop();
				return `{ ${ parts.map( p => `let _ = ${ p };` ).join( ' ' ) } ${ last } }`;
			}
			case 'AwaitExpression':
				return `await_val(${ this.expr( n.argument ) })?`;
			case 'YieldExpression':
				if ( n.delegate ) return `{ for y in iterate(&${ this.expr( n.argument ) })? { gen_items.push(y); } Val::Undef }`;
				return `{ gen_items.push(${ n.argument ? this.expr( n.argument ) : 'Val::Undef' }); Val::Undef }`;
			case 'SpreadElement':
				throw new Error( 'spread outside a list' );
			case 'MetaProperty':
				return 'Val::Undef';
			case 'Super':
				throw new Error( 'bare super' );
			default:
				throw new Error( `expression ${ n.type } in ${ this.jsPath }: ${ this.text( n ).slice( 0, 80 ) }` );
		}
	}

	arrayLit( elements ) {
		if ( elements.every( e => e && e.type !== 'SpreadElement' ) ) return `array(vec![${ elements.map( e => this.expr( e ) ).join( ', ' ) }])`;
		const parts = elements.map( e => ! e ? 'v.push(Val::Undef);' : e.type === 'SpreadElement' ? `spread_into(&mut v, &${ this.expr( e.argument ) }, ${ rustStr( this.text( e.argument ) ) })?;` : `v.push(${ this.expr( e ) });` );
		return `{ let mut v: Vec<Val> = vec![]; ${ parts.join( ' ' ) } array(v) }`;
	}

	args( list ) {
		if ( list.every( e => e.type !== 'SpreadElement' ) ) return `[${ list.map( e => this.expr( e ) ).join( ', ' ) }]`;
		const parts = list.map( e => e.type === 'SpreadElement' ? `spread_into(&mut v, &${ this.expr( e.argument ) }, ${ rustStr( this.text( e.argument ) ) })?;` : `v.push(${ this.expr( e ) });` );
		return `{ let mut v: Vec<Val> = vec![]; ${ parts.join( ' ' ) } v }`;
	}

	propKey( p ) {
		if ( p.computed ) return `&*key(&${ this.expr( p.key ) })?`;
		if ( p.key.type === 'Identifier' ) return rustStr( p.key.name );
		if ( p.key.type === 'Literal' ) return rustStr( String( p.key.value ) );
		throw new Error( 'key' );
	}

	objectLit( n ) {
		const lines = [];
		for ( const p of n.properties ) {
			if ( p.type === 'SpreadElement' ) {
				lines.push( `spread_obj(&o, &${ this.expr( p.argument ) })?;` );
				continue;
			}
			const k = this.propKey( p );
			if ( p.kind === 'get' ) lines.push( `o.obj().unwrap().define_accessor(${ k }, Some(${ this.fnExpr( p.value, '', true ) }), get_setter(&o, ${ k }), true);` );
			else if ( p.kind === 'set' ) lines.push( `o.obj().unwrap().define_accessor(${ k }, get_getter(&o, ${ k }), Some(${ this.fnExpr( p.value, '', true ) }), true);` );
			else {
				const name = p.key.type === 'Identifier' ? p.key.name : '';
				const v = p.value.type === 'FunctionExpression' || p.value.type === 'ArrowFunctionExpression' ? this.fnExpr( p.value, p.value.id ? p.value.id.name : name, p.method ) : this.expr( p.value );
				lines.push( `o.obj().unwrap().define(${ k }, ${ v }, true);` );
			}
		}
		return `{ let o = object(&[]); ${ lines.join( ' ' ) } o }`;
	}

	member( n ) {
		if ( n.object.type === 'Super' ) return `super_get(&parent, &this_v, ${ this.propKey( { computed: n.computed, key: n.property } ) })?`;
		const o = this.expr( n.object );
		if ( n.computed ) return `get_v(&${ o }, &${ this.expr( n.property ) })?`;
		return `get(&${ o }, ${ rustStr( n.property.name ) })?`;
	}

	// Optional chains: any `?.` link short-circuits the whole chain to undefined.
	chain( n ) {
		const t = this.t();
		const self = this;
		function go( e ) {
			if ( e.type === 'MemberExpression' ) {
				const o = go( e.object );
				const get = e.computed ? `get_v(&${ t }o, &${ self.expr( e.property ) })?` : `get(&${ t }o, ${ rustStr( e.property.name ) })?`;
				return `{ let ${ t }o = ${ o }; ${ e.optional ? `if ${ t }o.is_nullish() { break '${ t } Val::Undef; }` : '' } ${ get } }`;
			}
			if ( e.type === 'CallExpression' ) {
				const callee = e.callee;
				if ( callee.type === 'MemberExpression' ) {
					const o = go( callee.object );
					const key = callee.computed ? `&*key(&${ self.expr( callee.property ) })?` : rustStr( callee.property.name );
					return `{ let ${ t }r = ${ o }; ${ callee.optional ? `if ${ t }r.is_nullish() { break '${ t } Val::Undef; }` : '' } let ${ t }f = get(&${ t }r, ${ key })?; ${ e.optional ? `if ${ t }f.is_nullish() { break '${ t } Val::Undef; }` : '' } let a = ${ self.args( e.arguments ) }; call_checked(&${ t }f, &${ t }r, &a, ${ rustStr( self.text( callee ) ) })? }`;
				}
				const f = go( callee );
				return `{ let ${ t }f = ${ f }; ${ e.optional ? `if ${ t }f.is_nullish() { break '${ t } Val::Undef; }` : '' } let a = ${ self.args( e.arguments ) }; call_f(&${ t }f, &a, ${ rustStr( self.text( callee ) ) })? }`;
			}
			return self.expr( e );
		}
		return `('${ t }: { ${ go( n ) } })`;
	}

	callExpr( n ) {
		const callee = n.callee;
		if ( callee.type === 'Super' ) return `{ let a = ${ this.args( n.arguments ) }; super_init(&parent, &this_v, &a)?; Val::Undef }`;
		if ( callee.type === 'MemberExpression' && callee.object.type === 'Super' ) {
			return `{ let a = ${ this.args( n.arguments ) }; super_call(&parent, &this_v, ${ this.propKey( { computed: callee.computed, key: callee.property } ) }, &a)? }`;
		}
		if ( callee.type === 'MemberExpression' ) {
			const t = this.t();
			const key = callee.computed ? `&*key(&${ this.expr( callee.property ) })?` : rustStr( callee.property.name );
			return `{ let ${ t } = ${ this.expr( callee.object ) }; let f = get(&${ t }, ${ key })?; let a = ${ this.args( n.arguments ) }; call_checked(&f, &${ t }, &a, ${ rustStr( this.text( callee ) ) })? }`;
		}
		return `{ let f = ${ this.expr( callee ) }; let a = ${ this.args( n.arguments ) }; call_f(&f, &a, ${ rustStr( this.text( callee ) ) })? }`;
	}

	unary( n ) {
		const a = n.argument;
		switch ( n.operator ) {
			case '!': return `Val::Bool(!(${ this.expr( a ) }).truthy())`;
			case '-': return a.type === 'Literal' && typeof a.value === 'number' ? `Val::Num(${ numLit( - a.value ) })` : `neg(&${ this.expr( a ) })?`;
			case '+': return `plus(&${ this.expr( a ) })?`;
			case '~': return `bit_not(&${ this.expr( a ) })?`;
			case 'typeof':
				if ( a.type === 'Identifier' && ! this.scopeOf( a )?.lookup( a.name ) ) return `type_of(&glob(${ rustStr( a.name ) }))`;
				return `type_of(&${ this.expr( a ) })`;
			case 'void': return `{ let _ = ${ this.expr( a ) }; Val::Undef }`;
			case 'delete':
				if ( a.type === 'MemberExpression' ) {
					return a.computed ? `Val::Bool(delete_v(&${ this.expr( a.object ) }, &${ this.expr( a.property ) })?)` : `{ delete(&${ this.expr( a.object ) }, ${ rustStr( a.property.name ) })?; Val::Bool(true) }`;
				}
				return 'Val::Bool(true)';
		}
		throw new Error( 'unary ' + n.operator );
	}

	// [ read, write( valueExpr ) -> statements ] for an assignment target, with
	// the object and key evaluated once into temporaries.
	target( n, pre ) {
		if ( n.type === 'Identifier' ) return [ this.read( n.name, n ), v => this.write( n.name, n, v ) ];
		if ( n.type === 'MemberExpression' ) {
			const o = this.t();
			pre.push( `let ${ o } = ${ this.expr( n.object ) };` );
			if ( n.computed ) {
				const k = this.t();
				pre.push( `let ${ k } = key(&${ this.expr( n.property ) })?;` );
				return [ `get(&${ o }, &${ k })?`, v => `set(&${ o }, &${ k }, ${ v })?;` ];
			}
			const name = rustStr( n.property.name );
			return [ `get(&${ o }, ${ name })?`, v => `set(&${ o }, ${ name }, ${ v })?;` ];
		}
		throw new Error( 'target ' + n.type );
	}

	update( n ) {
		const pre = [];
		const [ read, write ] = this.target( n.argument, pre );
		const op = n.operator === '++' ? '+' : '-';
		const t = this.t();
		return `{ ${ pre.join( ' ' ) } let ${ t } = num(&${ read })?; ${ write( `Val::Num(${ t } ${ op } 1.0)` ) } Val::Num(${ n.prefix ? `${ t } ${ op } 1.0` : t }) }`;
	}

	binary( op, left, right ) {
		const a = this.expr( left ), b = this.expr( right );
		const fns = { '+': 'add', '-': 'sub', '*': 'mul', '/': 'div', '%': 'rem', '**': 'pow', '&': 'bit_and', '|': 'bit_or', '^': 'bit_xor', '<<': 'shl', '>>': 'shr', '>>>': 'ushr' };
		if ( fns[ op ] ) return `${ fns[ op ] }(&${ a }, &${ b })?`;
		const t = this.t();
		switch ( op ) {
			case '===': return `{ let ${ t } = ${ a }; Val::Bool(${ t }.strict_eq(&${ b })) }`;
			case '!==': return `{ let ${ t } = ${ a }; Val::Bool(!${ t }.strict_eq(&${ b })) }`;
			case '==': return `{ let ${ t } = ${ a }; Val::Bool(${ t }.loose_eq(&${ b })) }`;
			case '!=': return `{ let ${ t } = ${ a }; Val::Bool(!${ t }.loose_eq(&${ b })) }`;
			case '<': return `Val::Bool(lt(&${ a }, &${ b })?)`;
			case '>': return `Val::Bool(gt(&${ a }, &${ b })?)`;
			case '<=': return `Val::Bool(le(&${ a }, &${ b })?)`;
			case '>=': return `Val::Bool(ge(&${ a }, &${ b })?)`;
			case 'instanceof': return `Val::Bool(instance_of(&${ a }, &${ b })?)`;
			case 'in': return `Val::Bool(in_(&${ a }, &${ b })?)`;
		}
		throw new Error( 'binary ' + op );
	}

	assign( n ) {
		if ( n.left.type === 'ObjectPattern' || n.left.type === 'ArrayPattern' ) {
			const t = this.t();
			return `{ let ${ t } = ${ this.expr( n.right ) }; ${ this.pattern( n.left, t, 'assign' ) } ${ t } }`;
		}
		const pre = [];
		const [ read, write ] = this.target( n.left, pre );
		const t = this.t();
		const rhs = n.right.type === 'FunctionExpression' || n.right.type === 'ArrowFunctionExpression' ? this.fnExpr( n.right, n.left.type === 'Identifier' ? n.left.name : n.left.property?.name ?? '' ) : this.expr( n.right );
		if ( n.operator === '=' ) return `{ ${ pre.join( ' ' ) } let ${ t } = ${ rhs }; ${ write( `${ t }.clone()` ) } ${ t } }`;
		if ( n.operator === '||=' ) return `{ ${ pre.join( ' ' ) } let ${ t } = ${ read }; if ${ t }.truthy() { ${ t } } else { let ${ t } = ${ rhs }; ${ write( `${ t }.clone()` ) } ${ t } } }`;
		if ( n.operator === '&&=' ) return `{ ${ pre.join( ' ' ) } let ${ t } = ${ read }; if !${ t }.truthy() { ${ t } } else { let ${ t } = ${ rhs }; ${ write( `${ t }.clone()` ) } ${ t } } }`;
		if ( n.operator === '??=' ) return `{ ${ pre.join( ' ' ) } let ${ t } = ${ read }; if !${ t }.is_nullish() { ${ t } } else { let ${ t } = ${ rhs }; ${ write( `${ t }.clone()` ) } ${ t } } }`;
		const op = n.operator.slice( 0, - 1 );
		const fns = { '+': 'add', '-': 'sub', '*': 'mul', '/': 'div', '%': 'rem', '**': 'pow', '&': 'bit_and', '|': 'bit_or', '^': 'bit_xor', '<<': 'shl', '>>': 'shr', '>>>': 'ushr' };
		return `{ ${ pre.join( ' ' ) } let ${ t } = ${ fns[ op ] }(&${ read }, &${ rhs })?; ${ write( `${ t }.clone()` ) } ${ t } }`;
	}

	// Statements binding a pattern from the Rust value `src` ( kind: let / assign ).
	pattern( p, src, kind, node = p ) {
		const bind = ( id, value ) => kind === 'assign' ? ( id.type === 'Identifier' ? this.write( id.name, id, value ) : this.memberWrite( id, value ) ) : this.declareLocal( id.name, id, value );
		switch ( p.type ) {
			case 'Identifier':
				return bind( p, `${ src }.clone()` );
			case 'MemberExpression':
				return this.memberWrite( p, `${ src }.clone()` );
			case 'AssignmentPattern': {
				const t = this.t();
				return `let ${ t } = { let d = ${ src }.clone(); if d.is_undef() { ${ this.expr( p.right ) } } else { d } }; ${ this.pattern( p.left, t, kind ) }`;
			}
			case 'ObjectPattern': {
				const out = [];
				const used = [];
				for ( const prop of p.properties ) {
					if ( prop.type === 'RestElement' ) {
						const t = this.t();
						out.push( `let ${ t } = object_rest(&${ src }, &[${ used.join( ', ' ) }])?;` );
						out.push( this.pattern( prop.argument, t, kind ) );
						continue;
					}
					const t = this.t();
					const k = prop.computed ? `&*key(&${ this.expr( prop.key ) })?` : rustStr( prop.key.type === 'Identifier' ? prop.key.name : String( prop.key.value ) );
					if ( ! prop.computed ) used.push( k );
					out.push( `let ${ t } = destructure(&${ src }, ${ k }, ${ rustStr( this.sourceName( node ) ) })?;` );
					out.push( this.pattern( prop.value, t, kind ) );
				}
				if ( p.properties.length === 0 ) out.push( `destructure(&${ src }, "", "")?;` );
				return out.join( ' ' );
			}
			case 'ArrayPattern': {
				const items = this.t();
				const out = [ `let ${ items } = iterate_site(&${ src }, "")?;` ];
				p.elements.forEach( ( el, i ) => {
					if ( ! el ) return;
					const t = this.t();
					if ( el.type === 'RestElement' ) {
						out.push( `let ${ t } = array(${ items }.iter().skip(${ i }).cloned().collect());` );
						out.push( this.pattern( el.argument, t, kind ) );
					} else {
						out.push( `let ${ t } = ${ items }.get(${ i }).cloned().unwrap_or(Val::Undef);` );
						out.push( this.pattern( el, t, kind ) );
					}
				} );
				return out.join( ' ' );
			}
		}
		throw new Error( 'pattern ' + p.type );
	}

	sourceName( node ) {
		return node.__sourceName || 'object';
	}

	memberWrite( m, value ) {
		const pre = [];
		const [ , write ] = this.target( m, pre );
		return `{ ${ pre.join( ' ' ) } ${ write( value ) } }`;
	}

	// ---------------------------------------------------------------- functions

	paramLength( params ) {
		let n = 0;
		for ( const p of params ) {
			if ( p.type === 'AssignmentPattern' || p.type === 'RestElement' ) break;
			n ++;
		}
		return n;
	}

	// The captured cells a closure needs cloned in: bindings of enclosing
	// functions referenced in `node`.
	captures( node ) {
		const fnScope = this.scopeOf( node );
		const out = new Map();
		const self = this;
		const visit = ( n ) => {
			if ( ! n || typeof n !== 'object' ) return;
			if ( Array.isArray( n ) ) {
				for ( const x of n ) visit( x );
				return;
			}
			if ( n.type === 'Identifier' ) {
				const s = self.scopeOf( n );
				if ( s ) {
					const b = s.lookup( n.name );
					if ( b && b.scope !== self.moduleScope && b.captured && ! isInside( b.scope, fnScope ) ) out.set( b.id, b );
				}
			}
			for ( const k of Object.keys( n ) ) {
				if ( k === 'type' || k === 'start' || k === 'end' ) continue;
				const v = n[ k ];
				if ( v && typeof v === 'object' ) visit( v );
			}
		};
		visit( node );
		return [ ...out.values() ];
	}

	usesThis( node ) {
		let found = false;
		const visit = ( n, inArrow ) => {
			if ( found || ! n || typeof n !== 'object' ) return;
			if ( Array.isArray( n ) ) {
				for ( const x of n ) visit( x, inArrow );
				return;
			}
			if ( n.type === 'ThisExpression' || n.type === 'Super' ) {
				found = true;
				return;
			}
			if ( n.type === 'FunctionExpression' || n.type === 'FunctionDeclaration' || n.type === 'ClassExpression' || n.type === 'ClassDeclaration' ) return;
			for ( const k of Object.keys( n ) ) {
				if ( k === 'type' || k === 'start' || k === 'end' ) continue;
				const v = n[ k ];
				if ( v && typeof v === 'object' ) visit( v, inArrow );
			}
		};
		visit( node.body, true );
		return found;
	}

	// A function value: `func( name, length, move | this, args | … )` with
	// captured cells, `this` and the class parent cloned in.
	fnExpr( n, name, isMethod = false ) {
		const caps = this.captures( n );
		const arrow = n.type === 'ArrowFunctionExpression';
		const clones = caps.map( b => `let ${ this.local( b ) } = ${ this.local( b ) }.clone();` );
		if ( arrow && this.usesThis( n ) ) clones.push( 'let this_v = this_v.clone();' );
		if ( this.fnStack.length && this.fnStack[ this.fnStack.length - 1 ].hasParent ) clones.push( 'let parent = parent.clone();' );
		const body = this.fnBody( n, arrow );
		const ctor = ! arrow && ! isMethod && ! n.async && ! n.generator ? 'func_ctor' : 'func';
		return `{ ${ clones.join( ' ' ) } ${ ctor }(${ rustStr( name ) }, ${ this.paramLength( n.params ) }, move |this_in: &Val, args: &[Val]| -> R<Val> { ${ body } }) }`;
	}

	fnBody( n, arrow, extra = '' ) {
		const saved = { loops: this.loops, closureDepth: this.closureDepth };
		this.loops = [];
		this.closureDepth = 0;
		const top = this.fnStack[ this.fnStack.length - 1 ];
		this.fnStack.push( { node: n, generator: n.generator, async: n.async, hasParent: top ? top.hasParent : false } );
		const scope = this.scopeOf( n );
		const lines = [];
		if ( ! arrow ) lines.push( '#[allow(unused_variables)] let this_v = this_in.clone();' );
		lines.push( '#[allow(unused_variables)] let args = args;' );
		// Hoisted `var`s.
		for ( const b of scope.names.values() ) {
			if ( b.kind === 'var' ) {
				b.hoisted = true;
				lines.push( b.captured ? `let ${ this.local( b ) }: Var = var(Val::Undef);` : `#[allow(unused_mut)] let mut ${ this.local( b ) }: Val = Val::Undef;` );
			}
		}
		if ( n.id && n.type === 'FunctionExpression' && scope.names.get( n.id.name )?.kind === 'fname' ) {
			// A named function expression's own name: the callee, through the arguments' function.
			const b = scope.names.get( n.id.name );
			lines.push( b.captured ? `let ${ this.local( b ) }: Var = var(Val::Undef);` : `#[allow(unused_mut, unused_variables)] let mut ${ this.local( b ) }: Val = Val::Undef;` );
		}
		n.params.forEach( ( p, i ) => {
			if ( p.type === 'Identifier' ) lines.push( this.declareLocal( p.name, p, `arg(args, ${ i })` ) );
			else if ( p.type === 'RestElement' ) {
				const t = this.t();
				lines.push( `let ${ t } = array(args.iter().skip(${ i }).cloned().collect());` );
				lines.push( this.pattern( p.argument, t, 'let' ) );
			} else {
				const t = this.t();
				lines.push( `let ${ t } = arg(args, ${ i });` );
				lines.push( this.pattern( p, t, 'let' ) );
			}
		} );
		lines.push( extra );
		let body;
		if ( n.body.type === 'BlockStatement' ) {
			lines.push( this.predeclare( this.scopes.get( n.body ) ) );
			lines.push( this.hoistFunctions( n.body.body ) );
			lines.push( ...n.body.body.map( s => this.stmt( s ) ) );
			body = lines.join( '\n' ) + ( n.generator ? '\nOk(array(gen_items))' : '\n#[allow(unreachable_code)] Ok(Val::Undef)' );
		} else {
			body = lines.join( '\n' ) + `\nOk(${ this.expr( n.body ) })`;
		}
		if ( n.generator ) body = '#[allow(unused_mut)] let mut gen_items: Vec<Val> = vec![];\n' + body;
		if ( n.async ) body = `async_result((|| -> R<Val> { ${ body } })())`;
		this.fnStack.pop();
		this.loops = saved.loops;
		this.closureDepth = saved.closureDepth;
		return body;
	}

	hoistFunctions( stmts ) {
		const out = [];
		for ( const s of stmts ) {
			if ( s.type === 'FunctionDeclaration' ) out.push( this.declareLocal( s.id.name, s.id, this.fnExpr( s, s.id.name ) ) );
		}
		return out.join( '\n' );
	}

	// ---------------------------------------------------------------- classes

	classExpr( n, writeSelf = false ) {
		const name = n.id ? n.id.name : '';
		const t = this.t();
		const lines = [];
		const parentExpr = n.superClass ? this.expr( n.superClass ) : null;
		lines.push( `let parent: Val = ${ parentExpr ?? 'Val::Undef' };` );
		const ctor = n.body.body.find( m => m.kind === 'constructor' );
		const fields = n.body.body.filter( m => m.type === 'PropertyDefinition' && ! m.static );
		this.fnStack.push( { node: n, hasParent: true } );
		const fieldInit = fields.map( f => `set(&this_v, ${ this.propKey( f ) }, ${ f.value ? this.expr( f.value ) : 'Val::Undef' })?;` ).join( ' ' );
		let ctorFn;
		if ( ctor ) {
			// Fields initialize right after super() in a derived class, first otherwise.
			const value = ctor.value;
			const saveFields = fieldInit;
			const body = this.fnBody( value, false, n.superClass ? '' : saveFields );
			ctorFn = { length: this.paramLength( value.params ), body: n.superClass && fields.length ? body.replace( 'super_init(&parent, &this_v, &a)?;', `super_init(&parent, &this_v, &a)?; ${ saveFields }` ) : body, caps: this.captures( value ) };
		} else {
			ctorFn = { length: 0, body: `#[allow(unused_variables)] let this_v = this_in.clone(); ${ n.superClass ? 'super_init(&parent, &this_v, args)?;' : '' } ${ fieldInit } Ok(Val::Undef)`, caps: [] };
		}
		const clonesFor = ( caps ) => caps.map( b => `let ${ this.local( b ) } = ${ this.local( b ) }.clone();` ).join( ' ' );
		lines.push( `let ${ t } = { ${ clonesFor( ctorFn.caps ) } let parent = parent.clone(); Class::new(${ rustStr( name ) }, if parent.is_undef() { None } else { Some(cls(&parent)) }.as_ref(), ${ ctorFn.length }, move |this_in: &Val, args: &[Val]| -> R<Val> { ${ ctorFn.body } }) };` );
		if ( n.id && n.type === 'ClassExpression' ) {
			lines.push( this.declareLocal( n.id.name, n.id, `${ t }.ctor.clone()` ) );
		}
		// A declaration's binding exists before its static blocks run.
		if ( writeSelf ) lines.push( this.write( n.id.name, n.id, `${ t }.ctor.clone()` ) );
		for ( const m of n.body.body ) {
			if ( m.kind === 'constructor' ) continue;
			if ( m.type === 'StaticBlock' ) continue;
			if ( m.type === 'PropertyDefinition' ) continue;
			const k = this.propKey( m );
			const fname = m.key.type === 'Identifier' ? m.key.name : ( m.key.type === 'Literal' ? String( m.key.value ) : '' );
			const f = this.fnExpr( m.value, fname, true );
			const target = m.static ? `${ t }.ctor.obj().unwrap()` : `(&${ t }.prototype)`;
			if ( m.kind === 'get' ) lines.push( `{ let o = ${ target }; o.define_accessor(${ k }, Some(${ f }), accessor_part(o, ${ k }, false), false); }` );
			else if ( m.kind === 'set' ) lines.push( `{ let o = ${ target }; o.define_accessor(${ k }, accessor_part(o, ${ k }, true), Some(${ f }), false); }` );
			else lines.push( `${ target }.define(${ k }, ${ f }, false);` );
		}
		this.fnStack.pop();
		// Static fields and blocks run in order with `this` the constructor.
		const statics = n.body.body.filter( m => m.type === 'StaticBlock' || ( m.type === 'PropertyDefinition' && m.static ) );
		if ( statics.length ) {
			const saved = this.fnStack;
			this.fnStack = [ ...saved, { node: n, hasParent: true } ];
			const parts = [];
			for ( const m of statics ) {
				if ( m.type === 'PropertyDefinition' ) parts.push( `${ t }.ctor.obj().unwrap().define(${ this.propKey( m ) }, ${ m.value ? this.expr( m.value ) : 'Val::Undef' }, true);` );
				else parts.push( `{ ${ this.hoistFunctions( m.body ) } ${ m.body.map( s => this.stmt( s ) ).join( '\n' ) } }` );
			}
			this.fnStack = saved;
			lines.push( `let this_v = ${ t }.ctor.clone(); ${ parts.join( '\n' ) }` );
		}
		return `{ ${ lines.join( '\n' ) } ${ t }.ctor.clone() }`;
	}

	// ---------------------------------------------------------------- statements

	ret( value ) {
		if ( this.closureDepth > 0 ) return `return Ok(Ctl::Return(${ value }));`;
		const top = this.fnStack[ this.fnStack.length - 1 ];
		if ( top && top.generator ) return `{ let _ = ${ value }; return Ok(array(gen_items)); }`;
		return `return Ok(${ value });`;
	}

	stmt( n ) {
		switch ( n.type ) {
			case 'ExpressionStatement':
				return `let _ = ${ this.expr( n.expression ) };`;
			case 'VariableDeclaration':
				return n.declarations.map( d => {
					d.id.__sourceName = d.init ? this.text( d.init ) : '';
					const value = d.init ? ( ( d.init.type === 'FunctionExpression' || d.init.type === 'ArrowFunctionExpression' ) && d.id.type === 'Identifier' ? this.fnExpr( d.init, d.init.id ? d.init.id.name : d.id.name ) : ( d.init.type === 'ClassExpression' && ! d.init.id && d.id.type === 'Identifier' ? this.classExpr( { ...d.init, id: { type: 'Identifier', name: d.id.name } } ) : this.expr( d.init ) ) ) : 'Val::Undef';
					if ( d.id.type === 'Identifier' ) return this.declareLocal( d.id.name, d.id, value );
					const t = this.t();
					return `let ${ t } = ${ value }; ${ this.pattern( d.id, t, 'let' ) }`;
				} ).join( '\n' );
			case 'FunctionDeclaration':
				return ''; // hoisted
			case 'ClassDeclaration':
				return `${ this.declareLocal( n.id.name, n.id, 'Val::Undef' ) } let _ = ${ this.classExpr( n, true ) };`;
			case 'ReturnStatement':
				return this.ret( n.argument ? this.expr( n.argument ) : 'Val::Undef' );
			case 'IfStatement':
				return `if (${ this.expr( n.test ) }).truthy() { ${ this.stmt( n.consequent ) } }${ n.alternate ? ` else { ${ this.stmt( n.alternate ) } }` : '' }`;
			case 'BlockStatement':
				return `{ ${ this.predeclare( this.scopes.get( n ) ) } ${ this.hoistFunctions( n.body ) } ${ n.body.map( s => this.stmt( s ) ).join( '\n' ) } }`;
			case 'EmptyStatement':
				return '';
			case 'ThrowStatement':
				return `return Err(Exc(${ this.expr( n.argument ) }));`;
			case 'ForStatement':
				return this.forStmt( n );
			case 'ForOfStatement':
			case 'ForInStatement':
				return this.forOf( n );
			case 'WhileStatement':
				return this.loop( n, null, `if !(${ this.expr( n.test ) }).truthy() { break; }`, n.body, '' );
			case 'DoWhileStatement':
				return this.loop( n, null, '', n.body, `if !(${ this.expr( n.test ) }).truthy() { break; }` );
			case 'BreakStatement':
				return this.jump( 'break', n.label ? n.label.name : null );
			case 'ContinueStatement':
				return this.jump( 'continue', n.label ? n.label.name : null );
			case 'SwitchStatement':
				return this.switchStmt( n );
			case 'TryStatement':
				return this.tryStmt( n );
			case 'LabeledStatement': {
				this.pendingLabel = n.label.name;
				const body = n.body;
				if ( [ 'ForStatement', 'ForOfStatement', 'ForInStatement', 'WhileStatement', 'DoWhileStatement' ].includes( body.type ) ) return this.stmt( body );
				const label = `'L_${ n.label.name }`;
				this.loops.push( { label, cont: null, js: n.label.name, depth: this.closureDepth, block: true } );
				const out = `${ label }: { ${ this.stmt( body ) } }`;
				this.loops.pop();
				this.pendingLabel = null;
				return out;
			}
			case 'ImportDeclaration':
				return '';
			case 'ExportNamedDeclaration':
				if ( n.declaration ) return this.stmt( n.declaration );
				if ( n.source ) {
					const from = this.resolveModule( n.source.value );
					return n.specifiers.map( sp => `export(M, ${ rustStr( sp.exported.name ) }, imp(${ rustStr( from ) }, ${ rustStr( sp.local.name ) }));` ).join( '\n' );
				}
				// Local bindings export as they are written; imported ones re-export here.
				return n.specifiers.filter( sp => this.moduleScope.lookup( sp.local.name )?.kind === 'import' ).map( sp => `export(M, ${ rustStr( sp.exported.name ?? sp.exported.value ) }, ${ this.read( sp.local.name, sp.local ) });` ).join( '\n' );
			case 'ExportDefaultDeclaration': {
				const d = n.declaration;
				if ( ( d.type === 'ClassDeclaration' || d.type === 'FunctionDeclaration' ) && d.id ) return d.type === 'ClassDeclaration' ? this.stmt( d ) : '';
				const v = d.type === 'ClassDeclaration' ? this.classExpr( d ) : d.type === 'FunctionDeclaration' ? this.fnExpr( d, '' ) : this.expr( d );
				return `export(M, "default", ${ v });`;
			}
			case 'ExportAllDeclaration':
				if ( n.exported ) return `export(M, ${ rustStr( n.exported.name ?? n.exported.value ) }, module_namespace(${ rustStr( this.resolveModule( n.source.value ) ) }));`;
				return `reexport_all(${ rustStr( this.resolveModule( n.source.value ) ) }, M);`;
			default:
				throw new Error( `statement ${ n.type } in ${ this.jsPath }` );
		}
	}

	jump( kind, label ) {
		// The labeled statement, or the innermost loop ( or switch, for break ).
		let target = null;
		for ( let i = this.loops.length - 1; i >= 0; i -- ) {
			const l = this.loops[ i ];
			const ok = label ? l.js === label : ( kind === 'continue' ? l.cont !== null : ! l.block );
			if ( ok ) {
				target = l;
				break;
			}
		}
		if ( ! target ) throw new Error( `${ kind } without target in ${ this.jsPath }` );
		const rustLabel = kind === 'break' ? target.label : target.cont;
		if ( target.depth < this.closureDepth ) return `return Ok(Ctl::${ kind === 'break' ? 'Break' : 'Continue' }(Some(${ rustStr( rustLabel ) })));`;
		return kind === 'break' ? `break ${ rustLabel };` : `break ${ rustLabel };`;
	}

	loop( n, init, head, body, tail, update = '' ) {
		const id = this.tmp ++;
		const label = `'l${ id }`, cont = `'c${ id }`;
		const js = this.pendingLabel;
		this.pendingLabel = null;
		this.loops.push( { label, cont, js, depth: this.closureDepth } );
		const b = this.stmt( body );
		this.loops.pop();
		return `${ label }: loop { ${ head } ${ cont }: { ${ b } } ${ update } ${ tail } }`;
	}

	forStmt( n ) {
		let init = '';
		if ( n.init ) init = n.init.type === 'VariableDeclaration' ? this.stmt( n.init ) : `let _ = ${ this.expr( n.init ) };`;
		const head = n.test ? `if !(${ this.expr( n.test ) }).truthy() { break; }` : '';
		const update = n.update ? `let _ = ${ this.expr( n.update ) };` : '';
		return `{ ${ init } ${ this.loop( n, null, head, n.body, '', update ) } }`;
	}

	forOf( n ) {
		const id = this.tmp ++;
		const label = `'l${ id }`, cont = `'c${ id }`;
		const js = this.pendingLabel;
		this.pendingLabel = null;
		const item = `it${ id }`;
		let bind;
		if ( n.left.type === 'VariableDeclaration' ) {
			const d = n.left.declarations[ 0 ];
			bind = d.id.type === 'Identifier' ? this.declareLocal( d.id.name, d.id, `${ item }.clone()` ) : this.pattern( d.id, item, 'let' );
		} else {
			bind = n.left.type === 'Identifier' ? this.write( n.left.name, n.left, `${ item }.clone()` ) : this.pattern( n.left, item, 'assign' );
		}
		this.loops.push( { label, cont, js, depth: this.closureDepth } );
		const b = this.stmt( n.body );
		this.loops.pop();
		const source = n.type === 'ForOfStatement'
			? `iterate_site(&${ this.expr( n.right ) }, ${ rustStr( this.text( n.right ) ) })?`
			: `for_in_keys(&${ this.expr( n.right ) }).into_iter().map(Val::Str).collect::<Vec<_>>()`;
		return `${ label }: for ${ item } in ${ source } { ${ bind } ${ cont }: { ${ b } } }`;
	}

	switchStmt( n ) {
		const id = this.tmp ++;
		const label = `'s${ id }`;
		const d = `d${ id }`;
		this.loops.push( { label, cont: null, js: null, depth: this.closureDepth, isSwitch: true } );
		const tests = [];
		let defaultIndex = n.cases.length;
		n.cases.forEach( ( c, i ) => {
			if ( c.test ) tests.push( `if ${ d }.strict_eq(&${ this.expr( c.test ) }) { ${ i } }` );
			else defaultIndex = i;
		} );
		const start = `st${ id }`;
		const pick = tests.length ? `${ tests.join( ' else ' ) } else { ${ defaultIndex } }` : `${ defaultIndex }`;
		const pre = this.predeclare( this.scopes.get( n ) );
		const bodies = n.cases.map( ( c, i ) => `if ${ start } <= ${ i } { ${ this.hoistFunctions( c.consequent ) } ${ c.consequent.map( s => this.stmt( s ) ).join( '\n' ) } }` );
		this.loops.pop();
		return `${ label }: { let ${ d } = ${ this.expr( n.discriminant ) }; ${ pre } let ${ start }: usize = ${ pick }; ${ bodies.join( '\n' ) } }`;
	}

	tryStmt( n ) {
		const id = this.tmp ++;
		this.closureDepth ++;
		const body = this.stmt( n.block );
		let handler = null;
		if ( n.handler ) {
			const param = n.handler.param;
			let bind = '';
			if ( param ) bind = param.type === 'Identifier' ? this.declareLocal( param.name, param, `caught(e${ id })` ) : `let pe = caught(e${ id }); ${ this.pattern( param, 'pe', 'let' ) }`;
			handler = `${ bind } ${ this.stmt( n.handler.body ) }`;
		}
		this.closureDepth --;
		const fin = n.finalizer ? this.stmt( n.finalizer ) : '';
		// Dispatch the completion in the enclosing context.
		const outerReturn = this.closureDepth > 0 ? 'return Ok(Ctl::Return(v));' : ( this.fnStack.length && this.fnStack[ this.fnStack.length - 1 ].generator ? 'return Ok(array(gen_items));' : 'return Ok(v);' );
		const jumps = [];
		for ( const l of this.loops ) {
			for ( const [ kind, lab ] of [ [ 'Break', l.label ], [ 'Continue', l.cont ] ] ) {
				if ( ! lab ) continue;
				const here = l.depth < this.closureDepth ? `return Ok(Ctl::${ kind }(Some(${ rustStr( lab ) })))` : `break ${ lab }`;
				jumps.push( `Ctl::${ kind }(Some(${ rustStr( lab ) })) => { ${ here }; }` );
			}
		}
		const isFnLevel = this.fnStack.length > 0;
		return `{ #[allow(unused_mut)] let mut r${ id }: R<Ctl> = (|| -> R<Ctl> { ${ body } #[allow(unreachable_code)] Ok(Ctl::Normal) })();
${ handler !== null ? `if let Err(e${ id }) = r${ id } { r${ id } = (|| -> R<Ctl> { ${ handler } #[allow(unreachable_code)] Ok(Ctl::Normal) })(); }` : '' }
${ fin }
#[allow(unreachable_patterns)] match r${ id }? { Ctl::Normal => {} Ctl::Return(v) => { ${ isFnLevel ? outerReturn : 'let _ = v;' } } ${ jumps.join( ' ' ) } _ => {} } }`;
	}

	// ---------------------------------------------------------------- module

	module() {
		const body = this.ast.body;
		const lines = [];
		// Hoisted module functions, in order.
		for ( const st of body ) {
			const d = st.type === 'ExportNamedDeclaration' || st.type === 'ExportDefaultDeclaration' ? st.declaration : st;
			if ( d && d.type === 'FunctionDeclaration' && d.id ) {
				lines.push( this.write( d.id.name, d.id, this.fnExpr( d, d.id.name ) ) );
			}
		}
		for ( const st of body ) lines.push( this.stmt( st ) );
		const out = [];
		out.push( `//! Generated by tools/tsl/transpile.mjs from three r186 src/${ this.jsPath }. Do not edit.` );
		out.push( '#![allow(unused_variables, unused_mut, unused_braces, unused_parens, unreachable_code, unused_labels, non_snake_case, clippy::all)]' );
		out.push( 'use crate::reg::*;' );
		out.push( 'use crate::rt::ops::*;' );
		out.push( 'use crate::rt::*;' );
		out.push( '' );
		out.push( `const M: &str = ${ rustStr( this.jsPath ) };` );
		out.push( '' );
		out.push( 'thread_local! {' );
		out.push( `    static SLOTS: Vec<Var> = (0..${ Math.max( this.slots, 1 ) }).map(|_| var(Val::Undef)).collect();` );
		out.push( '}' );
		out.push( '#[allow(dead_code)]' );
		out.push( 'fn slot(i: usize) -> Val {' );
		out.push( '    SLOTS.with(|s| s[i].borrow().clone())' );
		out.push( '}' );
		out.push( '#[allow(dead_code)]' );
		out.push( 'fn set_slot(i: usize, v: Val) {' );
		out.push( '    SLOTS.with(|s| *s[i].borrow_mut() = v);' );
		out.push( '}' );
		out.push( '' );
		out.push( 'pub fn init() -> R<()> {' );
		out.push( '    #[allow(unused_variables)] let this_v = Val::Undef;' );
		out.push( lines.join( '\n' ) );
		out.push( '    Ok(())' );
		out.push( '}' );
		return out.join( '\n' ) + '\n';
	}

}

function isInside( scope, ancestor ) {
	for ( let s = scope; s; s = s.parent ) if ( s === ancestor ) return true;
	return false;
}

export function transpile( jsPath ) {
	const source = sourceOf( jsPath );
	const gen = new Gen( jsPath, source );
	const code = gen.module();
	const out = OUT + rustPath( jsPath ).join( '/' ) + '.rs';
	fs.mkdirSync( path.dirname( out ), { recursive: true } );
	fs.writeFileSync( out, code );
	return out;
}

if ( process.argv[ 1 ] && import.meta.url.endsWith( process.argv[ 1 ].split( '/' ).pop() ) ) {
	let ok = 0;
	for ( const p of process.argv.slice( 2 ) ) {
		try {
			transpile( p );
			ok ++;
		} catch ( e ) {
			console.error( p + ': ' + e.message );
		}
	}
	console.log( `${ ok } modules` );
}
