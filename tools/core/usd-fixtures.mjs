// USD layers for the USDLoader port's tests ( beside webgl_loader_usdz's
// saeukkang.usdz ): xformOp transforms, geometry primitives, cameras and lights;
// meshes with n-gons, holes, indexed and face-varying primvars, GeomSubsets,
// UsdPreviewSurface and OpenPBR materials with packaged textures; a skeleton
// with its animation and transform animation; and references between layers.
// `node tools/core/usd-fixtures.mjs` writes them to .cache/usd-fixtures/ as the
// bytes tests/usd.rs, tools/core/usd-oracle.mjs and the browser test parse.
import {deflateSync,zipSync,strToU8} from '../../.cache/three-r186/examples/jsm/libs/fflate.module.js';

// A w × h RGBA PNG ( stored filter rows, one IDAT ).
function png(w,h,pixel){
 const crcTable=Array.from({length:256},(_,n)=>{let c=n;for(let k=0;k<8;k++)c=c&1?0xedb88320^(c>>>1):c>>>1;return c>>>0;});
 const crc=b=>{let c=0xffffffff;for(const x of b)c=crcTable[(c^x)&255]^(c>>>8);return (c^0xffffffff)>>>0;};
 const chunk=(type,data)=>{const out=new Uint8Array(12+data.length);const dv=new DataView(out.buffer);dv.setUint32(0,data.length);out.set(strToU8(type),4);out.set(data,8);dv.setUint32(8+data.length,crc(out.subarray(4,8+data.length)));return out;};
 const ihdr=new Uint8Array(13);const dv=new DataView(ihdr.buffer);dv.setUint32(0,w);dv.setUint32(4,h);ihdr.set([8,6,0,0,0],8);
 const raw=new Uint8Array(h*(1+w*4));for(let y=0;y<h;y++)for(let x=0;x<w;x++)raw.set(pixel(x,y),y*(1+w*4)+1+x*4);
 const parts=[new Uint8Array([137,80,78,71,13,10,26,10]),chunk('IHDR',ihdr),chunk('IDAT',deflateSync(raw)),chunk('IEND',new Uint8Array())];
 const out=new Uint8Array(parts.reduce((a,p)=>a+p.length,0));let o=0;for(const p of parts){out.set(p,o);o+=p.length;}return out;
}

const transforms=`#usda 1.0
(
    defaultPrim = "Root"
    metersPerUnit = 0.01
    upAxis = "Z"
)

def Xform "Root"
{
    def Xform "A"
    {
        double3 xformOp:translate = (1, 2, 3)
        float3 xformOp:rotateXYZ = (10, 20, 30)
        float3 xformOp:scale = (2, 2, 2)
        uniform token[] xformOpOrder = ["xformOp:translate", "xformOp:rotateXYZ", "xformOp:scale"]

        def Cube "Box"
        {
            double size = 1.5
            double3 xformOp:translate = (0, 1, 0)
            uniform token[] xformOpOrder = ["xformOp:translate"]
        }
    }

    def Xform "B"
    {
        quatf xformOp:orient = (0.7071068, 0, 0.7071068, 0)
        float3 xformOp:scale = (-1, -1, -1)
        float3 xformOp:translate:pivot = (0.5, 0, 0)
        uniform token[] xformOpOrder = ["xformOp:translate:pivot", "xformOp:orient", "xformOp:scale", "!invert!xformOp:translate:pivot"]

        def Sphere "Ball"
        {
            double radius = 0.75
        }

        def Cylinder "Can"
        {
            double height = 3
            double radius = 0.5
            uniform token axis = "X"
        }

        def Cone "Hat"
        {
            uniform token axis = "Y"
        }

        def Capsule "Pill"
        {
            double height = 2
            double radius = 0.4
        }
    }

    def Xform "C"
    {
        matrix4d xformOp:transform = ( (1, 0, 0, 0), (0, 0, 1, 0), (0, -1, 0, 0), (4, 5, 6, 1) )
        uniform token[] xformOpOrder = ["xformOp:transform"]

        def Xform "D"
        {
            double xformOp:rotateY = 45
            double xformOp:rotateX = 30
            double xformOp:rotateZ = -15
            uniform token[] xformOpOrder = ["xformOp:rotateY", "xformOp:rotateX", "xformOp:rotateZ"]
        }
    }

    def Xform "E"
    {
        double3 xformOp:translate = (7, 8, 9)
        float3 xformOp:rotateXYZ = (5, 15, 25)
        float xformOp:scale = 3
        float3 xformOp:translate:pivot = (1, 1, 0)
    }

    def Camera "Cam"
    {
        float focalLength = 35
        float horizontalAperture = 36
        float verticalAperture = 24
        float horizontalApertureOffset = 2
        float focusDistance = 5
        float2 clippingRange = (0.1, 500)
        double3 xformOp:translate = (0, 0, 10)
        uniform token[] xformOpOrder = ["xformOp:translate"]
    }

    def Camera "Ortho"
    {
        token projection = "orthographic"
        float horizontalAperture = 100
        float verticalAperture = 50
        float verticalApertureOffset = 10
    }

    def DistantLight "Sun"
    {
        float inputs:intensity = 3
        color3f inputs:color = (1, 0.9, 0.8)
        float3 xformOp:rotateXYZ = (-45, 30, 0)
        uniform token[] xformOpOrder = ["xformOp:rotateXYZ"]
    }

    def SphereLight "Spot"
    {
        float inputs:intensity = 50
        float shaping:cone:angle = 30
        float shaping:cone:softness = 0.2
    }

    def SphereLight "Bulb"
    {
        float inputs:colorTemperature = 4000
    }

    def RectLight "Panel"
    {
        float inputs:width = 2
        float inputs:height = 3
    }

    def DiskLight "Disk"
    {
        float inputs:radius = 0.25
    }
}
`;

const mesh=`#usda 1.0
(
    defaultPrim = "World"
    upAxis = "Y"
)

def Xform "World"
{
    def Mesh "Fan"
    {
        int[] faceVertexCounts = [4, 3, 5]
        int[] faceVertexIndices = [0, 1, 2, 3, 3, 2, 4, 4, 2, 5, 6, 7]
        point3f[] points = [(0, 0, 0), (1, 0, 0), (1, 1, 0), (0, 1, 0), (0.5, 1.5, 0), (2, 1, 0), (2.5, 2, 0.2), (1, 2.5, 0.1)]
        texCoord2f[] primvars:st = [(0, 0), (1, 0), (1, 1), (0, 1), (0.5, 0.5)] (
            interpolation = "faceVarying"
        )
        int[] primvars:st:indices = [0, 1, 2, 3, 3, 2, 4, 4, 2, 1, 0, 3]
        color3f[] primvars:displayColor = [(0.2, 0.4, 0.6)]
        float[] primvars:displayOpacity = [0.5]
    }

    def Mesh "Split"
    {
        int[] faceVertexCounts = [4, 4, 3]
        int[] faceVertexIndices = [0, 1, 2, 3, 1, 4, 5, 2, 3, 2, 5]
        point3f[] points = [(0, 0, 0), (1, 0, 0), (1, 1, 0), (0, 1, 0), (2, 0, 0), (2, 1, 0)]
        normal3f[] normals = [(0, 0, 1), (0, 0.1, 1), (0, 0, 1), (0.1, 0, 1), (0, 0, 1), (0, 0, 1)] (
            interpolation = "vertex"
        )
        texCoord2f[] primvars:st = [(0, 0), (1, 0), (1, 1), (0, 1), (2, 0), (2, 1)] (
            interpolation = "vertex"
        )
        rel material:binding = </World/Looks/Textured>

        def GeomSubset "first"
        {
            uniform token elementType = "face"
            int[] indices = [0, 2]
            rel material:binding = </World/Looks/Red>
        }

        def GeomSubset "second"
        {
            uniform token elementType = "face"
            int[] indices = [1]
        }
    }

    def Mesh "Holed"
    {
        int[] faceVertexCounts = [4, 4]
        int[] faceVertexIndices = [0, 1, 2, 3, 4, 7, 6, 5]
        point3f[] points = [(0, 0, 0), (4, 0, 0), (4, 4, 0), (0, 4, 0), (1, 1, 0), (3, 1, 0), (3, 3, 0), (1, 3, 0)]
        int[] primvars:arnold:polygon_holes = [1, 0]
        double3 xformOp:translate = (5, 0, 0)
        uniform token[] xformOpOrder = ["xformOp:translate"]
    }

    def Mesh "Tri"
    {
        int[] faceVertexCounts = [3, 3]
        int[] faceVertexIndices = [0, 1, 2, 2, 1, 3]
        point3f[] points = [(0, 0, 1), (1, 0, 1), (0, 1, 1), (1, 1, 2)]
        normal3f[] primvars:normals = [(0, 0, 1), (0, 0, 1), (0, 0, 1), (0.3, 0, 1), (0, 0.3, 1), (0, 0, 1)] (
            interpolation = "faceVarying"
        )
        texCoord2f[] primvars:st1 = [(0, 0), (1, 0), (0, 1), (1, 1)]
        rel material:binding = </World/Looks/Pbr>
    }

    def Scope "Looks"
    {
        def Material "Red"
        {
            token outputs:surface.connect = </World/Looks/Red/Surface.outputs:surface>

            def Shader "Surface"
            {
                uniform token info:id = "UsdPreviewSurface"
                color3f inputs:diffuseColor = (0.8, 0.1, 0.1)
                float inputs:roughness = 0.4
                float inputs:metallic = 0.7
                float inputs:ior = 1.8
                float inputs:clearcoat = 0.3
                float inputs:clearcoatRoughness = 0.2
                color3f inputs:emissiveColor = (0.1, 0.2, 0.3)
                color3f inputs:specularColor = (0.9, 0.8, 0.7)
                float inputs:opacity = 0.6
                token outputs:surface
            }
        }

        def Material "Textured"
        {
            def Shader "Surface"
            {
                uniform token info:id = "UsdPreviewSurface"
                color3f inputs:diffuseColor.connect = </World/Looks/Textured/Tex.outputs:rgb>
                float inputs:roughness.connect = </World/Looks/Textured/Rough.outputs:g>
                float inputs:metallic.connect = </World/Looks/Textured/Rough.outputs:b>
                normal3f inputs:normal.connect = </World/Looks/Textured/Normal.outputs:rgb>
                color3f inputs:emissiveColor.connect = </World/Looks/Textured/Rough.outputs:rgb>
                float inputs:occlusion.connect = </World/Looks/Textured/Rough.outputs:r>
                float inputs:opacity.connect = </World/Looks/Textured/Tex.outputs:a>
                float inputs:opacityThreshold = 0.5
            }

            def Shader "Tex"
            {
                uniform token info:id = "UsdUVTexture"
                asset inputs:file = @textures/albedo.png@
                token inputs:wrapS = "mirror"
                token inputs:wrapT = "clamp"
                float4 inputs:scale = (0.5, 0.6, 0.7, 1)
                float4 inputs:bias = (0, 0.1, 0, 0)
                float2 inputs:st.connect = </World/Looks/Textured/Transform.outputs:result>
            }

            def Shader "Transform"
            {
                uniform token info:id = "UsdTransform2d"
                float2 inputs:in.connect = </World/Looks/Textured/Reader.outputs:result>
                float2 inputs:scale = (2, 3)
                float2 inputs:translation = (0.1, 0.2)
                float inputs:rotation = 30
            }

            def Shader "Reader"
            {
                uniform token info:id = "UsdPrimvarReader_float2"
                token inputs:varname = "st1"
            }

            def Shader "Rough"
            {
                uniform token info:id = "UsdUVTexture"
                asset inputs:file = @./textures/albedo.png@
                token inputs:wrapS = "repeat"
            }

            def Shader "Normal"
            {
                uniform token info:id = "UsdUVTexture"
                asset inputs:file = @textures/normal.png@
                float4 inputs:scale = (2, 2, 2, 1)
            }
        }

        def Material "Pbr"
        {
            def Shader "Surface"
            {
                uniform token info:id = "arnold:openpbr_surface"
                color3f inputs:base_color = (0.3, 0.6, 0.9)
                float inputs:base_metalness = 0.5
                float inputs:specular_roughness = 0.25
                color3f inputs:emission_color = (1, 0.5, 0)
                float inputs:emission_luminance = 2
                float inputs:transmission_weight = 0.4
                float inputs:transmission_depth = 0.3
                color3f inputs:transmission_color = (0.9, 0.95, 1)
                float inputs:geometry_opacity = 0.8
                float inputs:specular_ior = 1.6
                float inputs:coat_weight = 0.5
                float inputs:coat_roughness = 0.1
                float inputs:thin_film_weight = 0.7
                float inputs:thin_film_ior = 1.4
                float inputs:thin_film_thickness = 0.5
                float inputs:specular_weight = 0.9
                color3f inputs:specular_color = (1, 0.9, 0.9)
                float inputs:specular_roughness_anisotropy = 0.3
                normal3f inputs:geometry_normal.connect = </World/Looks/Pbr/Graph.outputs:normal>
            }

            def NodeGraph "Graph"
            {
                normal3f outputs:normal.connect = </World/Looks/Pbr/Graph/Img.outputs:out>

                def Shader "Img"
                {
                    uniform token info:id = "ND_image_vector3"
                    asset inputs:file = @textures/normal.png@
                }
            }
        }
    }
}
`;

const skel=`#usda 1.0
(
    timeCodesPerSecond = 30
    upAxis = "Y"
)

def SkelRoot "Rig"
{
    def Skeleton "Skel"
    {
        uniform token[] joints = ["root", "root/mid", "root/mid/tip"]
        uniform matrix4d[] bindTransforms = [( (1, 0, 0, 0), (0, 1, 0, 0), (0, 0, 1, 0), (0, 0, 0, 1) ), ( (1, 0, 0, 0), (0, 1, 0, 0), (0, 0, 1, 0), (0, 1, 0, 1) ), ( (1, 0, 0, 0), (0, 1, 0, 0), (0, 0, 1, 0), (0, 2, 0, 1) )]
        uniform matrix4d[] restTransforms = [( (1, 0, 0, 0), (0, 1, 0, 0), (0, 0, 1, 0), (0, 0, 0, 1) ), ( (1, 0, 0, 0), (0, 1, 0, 0), (0, 0, 1, 0), (0, 1, 0, 1) ), ( (0, 0, -1, 0), (0, 1, 0, 0), (1, 0, 0, 0), (0, 1, 0, 1) )]
        rel skel:animationSource = </Rig/Skel/Anim>

        def SkelAnimation "Anim"
        {
            uniform token[] joints = ["root", "root/mid", "root/mid/tip"]
            quatf[] rotations.timeSamples = {
                0: [(1, 0, 0, 0), (1, 0, 0, 0), (1, 0, 0, 0)],
                10: [(0.9238795, 0, 0, 0.3826834), (0.7071068, 0.7071068, 0, 0), (1, 0, 0, 0)],
            }
            float3[] translations.timeSamples = {
                0: [(0, 0, 0), (0, 1, 0), (0, 1, 0)],
                10: [(0, 0.5, 0), (0, 1, 0), (0, 1.5, 0)],
            }
            half3[] scales.timeSamples = {
                0: [(1, 1, 1), (1, 1, 1), (1, 1, 1)],
                10: [(1, 2, 1), (1, 1, 1), (0.5, 0.5, 0.5)],
            }
        }
    }

    def Mesh "Body"
    {
        int[] faceVertexCounts = [4, 4]
        int[] faceVertexIndices = [0, 1, 3, 2, 2, 3, 5, 4]
        point3f[] points = [(-0.5, 0, 0), (0.5, 0, 0), (-0.5, 1, 0), (0.5, 1, 0), (-0.5, 2, 0), (0.5, 2, 0)]
        int[] primvars:skel:jointIndices = [0, 1, 0, 1, 1, 2, 1, 2, 2, 0, 2, 1]
        float[] primvars:skel:jointWeights = [0.9, 0.1, 0.8, 0.2, 0.6, 0.4, 0.5, 0.5, 1, 0, 0.7, 0.3]
        matrix4d primvars:skel:geomBindTransform = ( (1, 0, 0, 0), (0, 1, 0, 0), (0, 0, 1, 0), (0, 0, 0.5, 1) )
        uniform token[] skel:joints = ["root/mid", "root", "root/mid/tip"]
        rel skel:skeleton = </Rig/Skel>
    }
}

def Xform "Mover"
{
    double3 xformOp:translate.timeSamples = {
        0: (0, 0, 0),
        24: (1, 2, 3),
    }
    float3 xformOp:rotateXYZ.timeSamples = {
        0: (0, 0, 0),
        12: (0, 90, 0),
    }
    float3 xformOp:scale.timeSamples = {
        6: (1, 1, 1),
        18: (2, 2, 2),
    }
    quatf xformOp:orient.timeSamples = {
        0: (1, 0, 0, 0),
        30: (0.7071068, 0, 0.7071068, 0),
    }
    uniform token[] xformOpOrder = ["xformOp:translate", "xformOp:rotateXYZ", "xformOp:scale"]
}
`;

const scene=`#usda 1.0
(
    defaultPrim = "Scene"
)

def Xform "Scene"
{
    def Xform "Prop" (
        prepend references = @./prop.usda@</Prop>
    )
    {
        double3 xformOp:translate = (0, 0, -2)
        uniform token[] xformOpOrder = ["xformOp:translate"]
    }

    def Scope "Pair" (
        prepend references = @parts/pair.usda@
    )
    {
        float3 xformOp:rotateXYZ = (0, 0, 90)
        uniform token[] xformOpOrder = ["xformOp:rotateXYZ"]
    }

    def Xform "Choice" (
        variants = {
            string shape = "round"
        }
        prepend variantSets = "shape"
    )
    {
        variantSet "shape" = {
            "round" {
                def Sphere "Thing"
                {
                }
            }
        }
    }

    def Mesh "Bound"
    {
        int[] faceVertexCounts = [3]
        int[] faceVertexIndices = [0, 1, 2]
        point3f[] points = [(0, 0, 0), (0, 1, 0), (1, 0, 0)]
    }
}
`;

const prop=`#usda 1.0
(
    defaultPrim = "Prop"
)

def Xform "Prop"
{
    def Mesh "Geo"
    {
        int[] faceVertexCounts = [3]
        int[] faceVertexIndices = [0, 1, 2]
        point3f[] points = [(0, 0, 0), (1, 0, 0), (0, 1, 0)]
        rel material:binding = </Prop/Mat>
    }

    def Material "Mat"
    {
        def Shader "S"
        {
            uniform token info:id = "UsdPreviewSurface"
            color3f inputs:diffuseColor = (0, 1, 0)
        }
    }
}
`;

const pair=`#usda 1.0

def Xform "Left"
{
    double3 xformOp:translate = (-1, 0, 0)
    uniform token[] xformOpOrder = ["xformOp:translate"]

    def Cube "Block"
    {
    }
}

def Xform "Right"
{
    double3 xformOp:translate = (1, 0, 0)
    uniform token[] xformOpOrder = ["xformOp:translate"]

    def Sphere "Ball"
    {
    }
}
`;

export function fixtures(){
 const albedo=png(4,4,(x,y)=>[x*60,y*60,(x+y)*30,255-x*20]);
 const normal=png(2,2,(x,y)=>[128+x*20,128+y*20,255,255]);
 const enc=t=>strToU8(t);
 return [
  {name:'transforms.usda',path:'',bytes:enc(transforms)},
  {name:'skel.usda',path:'',bytes:enc(skel)},
  {name:'mesh.usdz',path:'',bytes:zipSync({'mesh.usda':enc(mesh),'textures/albedo.png':albedo,'textures/normal.png':normal},{level:0})},
  {name:'scene.usdz',path:'',bytes:zipSync({'scene.usda':enc(scene),'prop.usda':enc(prop),'parts/pair.usda':enc(pair)},{level:6})},
 ];
}

if(import.meta.url===`file://${process.argv[1]}`){
 const {mkdir,writeFile}=await import('node:fs/promises');
 const out=new URL('../../.cache/usd-fixtures/',import.meta.url);
 await mkdir(out,{recursive:true});
 for(const f of fixtures())await writeFile(new URL(f.name,out),f.bytes);
 console.log(fixtures().map(f=>`${f.name}:${f.bytes.length}`).join(' '));
}
