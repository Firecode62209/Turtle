#version 450

layout(binding = 0) uniform CameraMatrix {
    mat4 view;
    mat4 proj;
} cam;

layout(location = 0) in vec3 position;

layout(location = 1) in vec4 inModelCol0;
layout(location = 2) in vec4 inModelCol1;
layout(location = 3) in vec4 inModelCol2;
layout(location = 4) in vec4 inModelCol3;
layout(location = 5) in vec3 inColor;

layout(location = 0) out vec4 fragColor;

void main()
{
mat4 model = mat4(inModelCol0, inModelCol1, inModelCol2, inModelCol3);
fragColor = vec4(inColor, 1.0);
gl_Position = cam.proj * cam.view * model * vec4(position, 1.0);
}