import * as T from '../../.cache/three-r186/src/Three.Core.js';
import {writeFileSync} from 'node:fs';
const fixtures=[];
for(const points of [[[0,0,0],[2,1,0]],[[0,0,0],[1,4,-2],[1,4,-2],[7,1,3],[-2,3,0]]])for(const closed of [false,true])for(const type of ['centripetal','chordal','catmullrom']){
 const curve=new T.CatmullRomCurve3(points.map(p=>new T.Vector3(...p)),closed,type,.3);
 fixtures.push({points,closed,type,tension:.3,samples:curve.getPoints(31).map(p=>p.toArray())});
}
writeFileSync('tests/fixtures/curves.json',JSON.stringify(fixtures));
