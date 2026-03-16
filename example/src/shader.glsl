// Raymarched scene: sphere on a reflective plane with soft shadows
// Demonstrates iResolution, iTime, and iMouse uniforms

float sdSphere(vec3 p, float r) { return length(p) - r; }
float sdPlane(vec3 p) { return p.y + 0.5; }

float scene(vec3 p) {
    float sphere = sdSphere(p - vec3(0.0, 0.3 + 0.2 * sin(iTime), 0.0), 0.5);
    float plane = sdPlane(p);
    return min(sphere, plane);
}

vec3 getNormal(vec3 p) {
    vec2 e = vec2(0.001, 0.0);
    return normalize(vec3(
        scene(p + e.xyy) - scene(p - e.xyy),
        scene(p + e.yxy) - scene(p - e.yxy),
        scene(p + e.yyx) - scene(p - e.yyx)
    ));
}

float softShadow(vec3 ro, vec3 rd, float mint, float maxt, float k) {
    float res = 1.0;
    float t = mint;
    for (int i = 0; i < 32; i++) {
        float h = scene(ro + rd * t);
        res = min(res, k * h / t);
        t += clamp(h, 0.02, 0.1);
        if (h < 0.001 || t > maxt) break;
    }
    return clamp(res, 0.0, 1.0);
}

float march(vec3 ro, vec3 rd) {
    float t = 0.0;
    for (int i = 0; i < 80; i++) {
        float d = scene(ro + rd * t);
        if (d < 0.001 || t > 20.0) break;
        t += d;
    }
    return t;
}

void mainImage(out vec4 fragColor, in vec2 fragCoord) {
    vec2 uv = (fragCoord - 0.5 * iResolution.xy) / iResolution.y;

    // Camera orbit controlled by mouse or time
    float angle = iMouse.x > 0.0
        ? (iMouse.x / iResolution.x - 0.5) * 6.28
        : iTime * 0.3;
    vec3 ro = vec3(2.5 * sin(angle), 1.0, 2.5 * cos(angle));
    vec3 target = vec3(0.0, 0.2, 0.0);
    vec3 fwd = normalize(target - ro);
    vec3 right = normalize(cross(fwd, vec3(0.0, 1.0, 0.0)));
    vec3 up = cross(right, fwd);
    vec3 rd = normalize(fwd + uv.x * right + uv.y * up);

    // Light
    vec3 lightDir = normalize(vec3(0.8, 0.8, -0.6));
    vec3 lightCol = vec3(1.0, 0.95, 0.85);
    vec3 skyCol = vec3(0.1, 0.12, 0.2);

    float t = march(ro, rd);
    vec3 col = skyCol;

    if (t < 20.0) {
        vec3 p = ro + rd * t;
        vec3 n = getNormal(p);

        // Material: checkerboard for the floor, solid for the sphere
        vec3 matCol;
        if (p.y < -0.49) {
            float checker = mod(floor(p.x * 2.0) + floor(p.z * 2.0), 2.0);
            matCol = mix(vec3(0.15, 0.15, 0.2), vec3(0.4, 0.4, 0.45), checker);
        } else {
            matCol = vec3(0.3, 0.5, 0.9);
        }

        // Diffuse + specular lighting
        float diff = max(dot(n, lightDir), 0.0);
        float shadow = softShadow(p + n * 0.01, lightDir, 0.02, 5.0, 16.0);
        vec3 h = normalize(lightDir - rd);
        float spec = pow(max(dot(n, h), 0.0), 32.0);

        // Ambient occlusion (simple)
        float ao = 0.5 + 0.5 * n.y;

        col = matCol * (0.15 * ao + diff * shadow * lightCol)
            + spec * shadow * lightCol * 0.5;
    }

    // Gamma correction
    col = pow(col, vec3(0.4545));
    fragColor = vec4(col, 1.0);
}
