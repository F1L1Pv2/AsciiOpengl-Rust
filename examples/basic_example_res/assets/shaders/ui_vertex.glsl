#version 150

in vec3 position;
in vec3 normal;

// uniform vec2 ui_position;

in vec2 tex_coords;
out vec2 v_tex_coords;

//ui shader

void main() {
  v_tex_coords = tex_coords;
  
  gl_Position = vec4(position.x-0.5, position.y-1.0, position.z, 1.0);
}