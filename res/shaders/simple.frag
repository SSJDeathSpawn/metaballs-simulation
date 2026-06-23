#version 430 core

struct Ball {
    vec2 pos;
    float radius;
    float padding1;

    vec3 color;
    float padding2;
};

layout(std430, binding = 0) buffer BallBuffer {
    Ball balls[];
};

uniform int ball_count;

out vec4 frag_color;

void main() {

    vec2 p = gl_FragCoord.xy;

    float total_weight = 0.0;
    vec3 total_color = vec3(0.0);

    for(int i=0; i<ball_count; i++) {
        vec2 d = p - balls[i].pos;
        float w = balls[i].radius * balls[i].radius / (dot(d,d) + 1.0);

        total_weight += w;
        total_color += w * balls[i].color;
    }

    if (total_weight > 1.0 && total_weight < 1.2)
        frag_color = vec4(total_color / total_weight, 1.0);
    else
        frag_color = vec4(0.0,0.0,0.0,1.0);
}
