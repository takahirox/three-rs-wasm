// The original examples' UI on the gallery's ports: their stats.js, lil-gui
// panels and WebGPU Inspector, built with the same libraries ( three r186's
// examples/jsm, from the CDN the TSL editor already uses ) from the structure
// each pinned page builds ( specs.json, tools/gallery/ui/extract.mjs ). An
// example's page carries its own part of the records ( #gallery-ui:
// tools/gallery/pages.mjs ).
// Controllers write to proxy objects whose properties forward to the port's
// parameters ( `app.tsl_parameter` ), matched to the gallery's control list
// by name.
export const LIBS = 'https://cdn.jsdelivr.net/npm/three@0.186.0/examples/jsm/';

// The page's records: { spec ( specs.json ), page ( pages.json ), aliases
// ( aliases.json: the gallery's names for controllers whose original names
// differ, "folder/name" or "name" ), extras ( a module of extras/ ) }.
function pageRecords() {

	const element = document.getElementById( 'gallery-ui' );
	return element ? JSON.parse( element.textContent ) : {};

}

// Color.getHex() → the linear components three's Color holds ( sRGB decode ).
function linearColor( hex ) {

	const srgb = v => ( v < 0.04045 ? v * 0.0773993808 : Math.pow( v * 0.9478672986 + 0.0521327014, 2.4 ) );
	return { r: srgb( ( hex >> 16 & 255 ) / 255 ), g: srgb( ( hex >> 8 & 255 ) / 255 ), b: srgb( ( hex & 255 ) / 255 ) };

}

// A recorded value back into a JavaScript value.
function revive( v ) {

	if ( v && typeof v === 'object' ) {

		if ( v.undefined ) return undefined;
		if ( v.number !== undefined ) return Number( v.number );
		if ( v.color !== undefined ) return linearColor( v.color );
		if ( v.function !== undefined ) return () => {};
		if ( Array.isArray( v ) ) return v.map( revive );
		const out = {};
		for ( const k of Object.keys( v ) ) out[ k ] = revive( v[ k ] );
		return out;

	}

	return v;

}

// Names compared without case, spaces or punctuation ( "instanceCount" and
// "instance count" ).
const key = s => String( s ?? '' ).toLowerCase().replace( /[^a-z0-9]/g, '' );

// What a controller is called in the gallery's lists: its name, its
// property, and either with its folder's title or the title's first word in
// front ( "Local Clipping" / "Enabled" → "Local Enabled" ).
function candidates( node, folder ) {

	const names = [ node.name, node.property ];
	if ( folder ) {

		const first = String( folder ).split( /\s+/ )[ 0 ];
		for ( const n of [ node.name, node.property ] ) names.push( `${ folder } ${ n }`, `${ first } ${ n }` );

	}

	return names.filter( n => n !== undefined && n !== null );

}

// The gallery's controls of an example: [ name, min, max, value, step ]
// ( see example.js ), and the inputs of its own panel. A binding converts a
// controller's value to the parameter the port reads, or sets the panel's
// input and fires its events.
export function bindings( controls, app, settings, aliases = {} ) {

	const used = new Set();
	const usedInputs = new Set();
	const unbound = [];
	const find = ( names ) => {

		for ( const exact of [ true, false ] ) {

			for ( const name of names ) {

				const i = controls.findIndex( ( c, i ) => c && ! used.has( i ) && ( exact ? c[ 0 ] === name : key( c[ 0 ] ) === key( name ) ) );
				if ( i !== - 1 ) {

					used.add( i );
					return i;

				}

			}

		}

		return - 1;

	};

	// The gallery panel's labelled inputs and buttons.
	const panel = () => settings ? [
		...[ ...settings.querySelectorAll( 'label' ) ].map( label => ( { element: label, input: label.querySelector( 'input,select' ), name: label.firstChild?.textContent ?? label.textContent } ) ),
		...[ ...settings.querySelectorAll( 'button' ) ].map( button => ( { element: button, input: button, name: button.textContent } ) )
	].filter( e => e.input && ! e.input.id.startsWith( 'particle-' ) ) : [];
	const findInput = names => {

		const inputs = panel();
		for ( const name of names ) {

			const found = inputs.find( e => ! usedInputs.has( e.input ) && key( e.name ) === key( name ) );
			if ( found ) {

				usedInputs.add( found.input );
				return found;

			}

		}

		return null;

	};

	const fire = ( input, value ) => {

		if ( input.tagName === 'BUTTON' ) return input.click();
		if ( input.type === 'checkbox' ) input.checked = !! value;
		else if ( input.tagName === 'SELECT' ) {

			const option = [ ...input.options ].find( o => o.textContent === String( value ) || o.value === String( value ) );
			if ( option ) input.value = option.value;

		} else input.value = String( value );
		input.dispatchEvent( new Event( 'input', { bubbles: true } ) );
		input.dispatchEvent( new Event( 'change', { bubbles: true } ) );

	};

	return {
		unbound,
		free: i => ! used.has( i ),
		panelInputs: usedInputs,
		bind( node, folder ) {

			const alias = aliases[ `${ folder }/${ node.name }` ] ?? aliases[ node.name ];
			const names = [ ...( alias ? [ alias ] : [] ), ...candidates( node, folder ) ];
			const i = find( names );
			if ( i === - 1 ) {

				const found = findInput( names );
				if ( found ) return value => fire( found.input, value );
				unbound.push( node.name ?? node.property );
				return null;

			}

			const [ , min ] = controls[ i ];
			return ( value ) => {

				let v;
				if ( min === null ) v = 1;
				else if ( Array.isArray( min ) ) {

					// An option: its name in the gallery's list, else its value.
					const names = node.names ?? ( Array.isArray( node.params?.[ 0 ] ) ? node.params[ 0 ] : null );
					const values = node.values ?? names;
					const name = names && values ? names[ values.findIndex( x => x === value ) ] : value;
					v = Math.max( 0, min.findIndex( x => String( x ) === String( name ) ) );

				} else if ( typeof min === 'boolean' ) v = Number( value );
				else if ( typeof min === 'string' ) v = typeof value === 'string' ? parseInt( value.replace( '#', '' ), 16 ) : value;
				else v = Number( value );
				app.tsl_parameter( i, v );

			};

		}
	};

}

// stats.js where the original created it, updated around the port's frames.
export async function installStats( specs ) {

	if ( ! specs.length ) return [];
	const { default: Stats } = await import( LIBS + 'libs/stats.module.js' );
	const all = specs.map( spec => {

		const stats = new Stats();
		for ( const i of spec.shows ) stats.showPanel( i );
		// The original's own changes to the panel's style ( position, offsets ).
		if ( spec.place?.style ) stats.dom.setAttribute( 'style', spec.place.style );
		document.body.appendChild( stats.dom );
		return { stats, spec };

	} );
	const begin = window.galleryFrameBegin, end = window.galleryFrameEnd;
	window.galleryFrameBegin = () => {

		begin?.();
		for ( const { stats, spec } of all ) if ( spec.begin ) stats.begin();

	};

	window.galleryFrameEnd = () => {

		end?.();
		for ( const { stats, spec } of all ) {

			if ( spec.begin ) stats.end();
			else stats.update();

		}

	};

	return all.map( a => a.stats );

}

let lilModule = null;
export async function loadLil() {

	lilModule ??= import( LIBS + 'libs/lil-gui.module.min.js' );
	return lilModule;

}

// The classes and style the page gave a panel element itself.
function dress( element, node ) {

	if ( node.classes !== undefined ) {

		const unbound = element.hasAttribute( 'data-unbound' );
		element.className = node.classes;
		if ( unbound ) element.dataset.unbound = '';

	}

	if ( node.style !== undefined && node.style !== null ) element.setAttribute( 'style', node.style );

}

// Elements the page added to a folder ( live readouts: see extras/ ).
function addExtras( gui, extras = [] ) {

	for ( const { index, html } of extras ) {

		const template = document.createElement( 'template' );
		template.innerHTML = html;
		const element = template.content.firstElementChild;
		gui.$children.insertBefore( element, gui.$children.children[ index ] ?? null );

	}

}

// The recorded lil-gui panels.
export async function installGuis( specs, binder ) {

	if ( ! specs.length ) return [];
	const { GUI } = await loadLil();
	const objects = new Map();
	const proxy = ( node, apply ) => {

		let object = objects.get( node.object );
		if ( object === undefined ) objects.set( node.object, object = {} );
		object[ node.property ] = revive( node.value );
		if ( node.type === 'function' ) object[ node.property ] = () => apply?.( 1 );
		return object;

	};

	const build = ( gui, nodes, title = null ) => {

		for ( const node of nodes ) {

			if ( node.kind === 'folder' ) {

				const folder = gui.addFolder( node.title );
				build( folder, node.children, node.title );
				if ( node.closed ) folder.close();
				if ( node.hidden ) folder.hide();
				dress( folder.domElement, node );
				addExtras( folder, node.extras );
				continue;

			}

			// A disabled controller ( a readout ) needs no parameter.
			const apply = node.disabled ? null : binder.bind( node, title );
			const object = proxy( node, apply );
			let c;
			if ( node.type === 'color' ) {

				c = gui.addColor( object, node.property, node.rgbScale );

			} else if ( node.type === 'option' ) {

				const values = revive( node.values ), names = revive( node.names );
				const plain = values.every( ( v, i ) => String( v ) === names[ i ] );
				c = gui.add( object, node.property, plain ? values : Object.fromEntries( names.map( ( n, i ) => [ n, values[ i ] ] ) ) );

			} else {

				c = gui.add( object, node.property );
				if ( node.type === 'number' ) {

					const n = v => revive( v );
					if ( n( node.min ) !== undefined ) c.min( n( node.min ) );
					if ( n( node.max ) !== undefined ) c.max( n( node.max ) );
					if ( n( node.step ) !== undefined ) c.step( n( node.step ) );
					if ( n( node.decimals ) !== undefined ) c.decimals( n( node.decimals ) );

				}

			}

			if ( node.name !== node.property ) c.name( node.name );
			if ( node.disabled ) c.disable();
			if ( node.hidden ) c.hide();
			if ( apply && node.type !== 'function' ) c.onChange( apply );
			dress( c.domElement, node );
			// The port has no parameter for it: shown, but disabled.
			if ( ! apply && ! node.disabled ) {

				c.disable();
				c.domElement.dataset.unbound = '';
				c.domElement.title = 'Rust版では未対応';

			}

		}

	};

	return specs.map( spec => {

		const autoPlace = spec.classes.includes( 'autoPlace' );
		const gui = new GUI( { title: spec.title, autoPlace } );
		if ( ! autoPlace ) document.body.appendChild( gui.domElement );
		if ( spec.place.style ) gui.domElement.setAttribute( 'style', spec.place.style );
		build( gui, spec.children );
		addExtras( gui, spec.extras );
		if ( spec.closed ) gui.close();
		if ( spec.hidden ) gui.hide();
		return gui;

	} );

}

// What the Inspector reads from three's renderer, from the port: the frame
// clock, the GPU memory the page allocated ( gpu-memory.js ) and its passes'
// GPU time ( gpu-timing.js; none without timestamp queries ).
function rendererFacade( canvas, timing ) {

	const nodeFrame = { frameId: 0, time: 0, deltaTime: 0, lastTime: performance.now() };
	const frameOf = uid => Number( uid.split( ':f' ).pop() );
	return {
		isWebGPURenderer: true,
		domElement: canvas,
		_nodes: { nodeFrame },
		info: { frame: 0, render: { frame: 0 }, compute: { frame: 0 }, memory: window.galleryGpuMemory ?? {} },
		backend: {
			isWebGPUBackend: true,
			trackTimestamp: false,
			hasTimestamp: timing !== null,
			getTimestampFrames: type => type === 'render' && timing ? [ ...timing.results.keys() ] : [],
			hasTimestampQuery: uid => timing !== null && timing.results.has( frameOf( uid ) ),
			getTimestamp: uid => timing?.results.get( frameOf( uid ) ) ?? 0
		},
		hasInitialized: () => true,
		init: async () => {},
		hasFeature: name => name === 'timestamp-query' && timing !== null,
		resolveTimestampsAsync: async () => timing?.settled(),
		update() {

			const now = performance.now();
			nodeFrame.deltaTime = ( now - nodeFrame.lastTime ) / 1000;
			nodeFrame.lastTime = now;
			nodeFrame.time += nodeFrame.deltaTime;
			nodeFrame.frameId ++;
			this.info.frame = nodeFrame.frameId;

		}
	};

}

// The WebGPU Inspector ( three r186's own ), its parameter groups as the
// original made them, and one render per frame: the port's frame.
export async function installInspector( spec, binder, canvas ) {

	const { Inspector } = await import( LIBS + 'inspector/Inspector.js' );
	const inspector = new Inspector( revive( spec.options ) ?? {} );
	const { installGpuTiming } = await import( './gpu-timing.js' );
	const timing = installGpuTiming( window.galleryGpuDevice );
	const renderer = rendererFacade( canvas, timing );
	renderer.inspector = inspector;
	inspector.setRenderer( renderer );
	const objects = new Map();
	const build = ( group, nodes, title = null ) => {

		for ( const node of nodes ) {

			if ( node.kind === 'folder' ) {

				const folder = group.addFolder( node.title );
				build( folder, node.children, node.title );
				for ( const call of node.calls ?? [] ) folder[ call ]();
				continue;

			}

			const apply = binder.bind( node, title );
			let object = objects.get( node.object );
			if ( object === undefined ) objects.set( node.object, object = {} );
			object[ node.property ] = node.type === 'button' ? () => apply?.( 1 ) : revive( node.value );
			const params = revive( node.params ) ?? [];
			const method = { slider: 'addSlider', number: 'addNumber', boolean: 'addBoolean', select: 'addSelect', color: 'addColor', string: 'addString', button: 'addButton' }[ node.type ];
			const editor = group[ method ]( object, node.property, ...params );
			if ( node.name !== node.property ) editor.name( node.name );
			if ( apply && node.type !== 'button' ) editor.addEventListener( 'change', ( { value } ) => apply( value ) );
			if ( node.hidden ) editor.dispatchEvent( { type: 'hide' } );
			if ( ! apply ) editor.__subItem?.domElement?.setAttribute( 'data-unbound', '' );

		}

	};

	for ( const g of spec.groups ) {

		const group = inspector.createParameters( g.title );
		build( group, g.children, g.title );
		for ( const call of g.calls ?? [] ) group[ call ]();

	}

	for ( const [ name, ...args ] of spec.calls ) {

		if ( name === 'setActiveTab' ) inspector.profiler.setActiveTab( args[ 0 ] );
		else inspector[ name ]?.( ...args );

	}

	if ( ! spec.parametersVisible ) inspector.parameters.hide();
	const begin = window.galleryFrameBegin, end = window.galleryFrameEnd;
	window.galleryFrameBegin = () => {

		begin?.();
		if ( inspector.isRunning ) inspector.finish();
		renderer.update();
		inspector.begin();
		inspector.beginRender( `r:0:f${ renderer._nodes.nodeFrame.frameId }`, { name: '', isScene: true }, null, null );
		timing?.begin();

	};

	window.galleryFrameEnd = () => {

		timing?.end( renderer._nodes.nodeFrame.frameId );
		inspector.finishRender();
		end?.();

	};

	return inspector;

}

// The original page's chrome: its stylesheet in place of the gallery's, the
// <style> blocks of its head and its #info ( pages.json, tools/gallery/ui/page.py ).
// Elements the port already made ( same id ) are not added twice.
export async function installPage( page ) {

	if ( ! page ) return null;
	// The originals are English pages: their generic fonts resolve as theirs do.
	document.documentElement.lang = 'en';
	if ( page.css ) {

		const link = document.createElement( 'link' );
		link.rel = 'stylesheet';
		link.href = new URL( `css/${ page.css }.css`, import.meta.url );
		const loaded = new Promise( r => {

			link.onload = r;
			link.onerror = r;

		} );
		document.head.append( link );
		await loaded;
		document.querySelector( 'link[href="../files/main.css"]' )?.remove();

	}

	for ( const css of page.styles ) {

		const style = document.createElement( 'style' );
		style.textContent = css;
		document.head.append( style );

	}

	if ( page.info && ! document.querySelector( '#info' ) ) {

		const template = document.createElement( 'template' );
		template.innerHTML = page.info;
		const info = template.content.firstElementChild;
		for ( const element of info.querySelectorAll( '[id]' ) ) if ( document.getElementById( element.id ) ) element.remove();
		document.body.prepend( info );

	}

	return page;

}

// The example's recorded UI on the gallery page. The gallery's own control
// panel keeps only the controls no original controller stands for.
export async function installUi( app, settings ) {

	const { spec, page, aliases, extras } = pageRecords();
	await installPage( page );
	if ( ! spec || spec.error ) return null;
	const controls = window.galleryControls ?? [];
	const binder = bindings( controls, app, settings, aliases );
	const stats = await installStats( spec.stats );
	const guis = await installGuis( spec.guis, binder );
	if ( extras ) ( await import( `./extras/${ extras }.js` ) ).default( guis );
	const canvas = document.querySelector( 'canvas' );
	const inspectors = [];
	for ( const i of spec.inspectors ) inspectors.push( await installInspector( i, binder, canvas ) );
	window.galleryInspectors = inspectors;
	const bound = new Set( [ ...controls.keys() ].filter( i => ! binder.free( i ) ) );
	if ( settings && ( guis.length || inspectors.length ) ) {

		for ( const i of bound ) document.querySelector( `#particle-${ i }` )?.closest( 'label,button' )?.remove();
		// The gallery panel's own inputs stay in the page ( its handlers read
		// them ) but out of view once an original controller drives them.
		for ( const input of binder.panelInputs ) ( input.closest( 'label' ) ?? input ).hidden = true;
		settings.hidden = [ ...settings.querySelectorAll( 'label,button' ) ].every( e => e.hidden || e.closest( '[hidden]' ) );

	}

	document.body.dataset.uiUnbound = binder.unbound.join( '|' );
	document.body.dataset.ui = 'ready';
	return { stats, guis, inspectors };

}
