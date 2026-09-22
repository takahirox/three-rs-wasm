// Adapted from Three.js r186 WoodNodeMaterial.js (MIT).

    fn wood_voronoi(x: vec3<f32>) -> f32
    {
        let smoothness=0.5;let randomness=1.0;
        let p = floor(x);
        let f = fract(x);

        var res = 0.0;
        var totalWeight = 0.0;

        for (var k = -1; k <= 1; k++)
        {
            for (var j = -1; j <= 1; j++)
            {
                for (var i = -1; i <= 1; i++)
                {
                    let b = vec3<f32>(f32(i), f32(j), f32(k));
                    let hashOffset = wood_hash3d(p + b) * randomness;
                    let r = b - f + hashOffset;
                    let d = length(r);

                    let weight = exp(-d * d / max(smoothness * smoothness, 0.001));
                    res += d * weight;
                    totalWeight += weight;
                }
            }
        }

        if (totalWeight > 0.0)
        {
            res /= totalWeight;
        }

        return smoothstep(0.0, 1.0, res);
    }

    fn wood_hash3d(p: vec3<f32>) -> vec3<f32>
    {
        var p3 = fract(p * vec3<f32>(0.1031, 0.1030, 0.0973));
        p3 += dot(p3, p3.yzx + 33.33);
        return fract((p3.xxy + p3.yzz) * p3.zyx);
    }
