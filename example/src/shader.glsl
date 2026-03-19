// "Ether" by nimitz (2014) — original: https://www.shadertoy.com/view/MsjSW3
// License: CC BY-NC-SA 3.0 (non-commercial use with attribution)
// Mouse X/Y wobble the xz/xy rotation angles, layered on top of the time animation.

#define t iTime
mat2 m(float a){float c=cos(a), s=sin(a);return mat2(c,-s,s,c);}
float map(vec3 p){
    vec2 mouse = iMouse.xy / iResolution.xy;
    p.xz *= m(t*0.4 + mouse.x * 6.28318);
    p.xy *= m(t*0.3 + mouse.y * 6.28318);
    vec3 q = p*2.+t;
    return length(p+vec3(sin(t*0.7)))*log(length(p)+1.) + sin(q.x+sin(q.z+sin(q.y)))*0.5 - 1.;
}

void mainImage( out vec4 fragColor, in vec2 fragCoord ){
    vec2 p = fragCoord.xy/iResolution.y - vec2(.9,.5);
    vec3 cl = vec3(0.);
    float d = 2.5;
    for(int i=0; i<=5; i++)    {
        vec3 p = vec3(0,0,5.) + normalize(vec3(p, -1.))*d;
        float rz = map(p);
        float f =  clamp((rz - map(p+.1))*0.5, -.1, 1. );
        vec3 l = vec3(0.1,0.3,.4) + vec3(5., 2.5, 3.)*f;
        cl = cl*l + smoothstep(2.5, .0, rz)*.7*l;
        d += min(rz, 1.);
    }
    fragColor = vec4(cl, 1.);
}
