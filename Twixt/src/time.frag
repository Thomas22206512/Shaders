#version 100
precision lowp float;

varying vec4 color;
varying vec2 uv;

uniform sampler2D Texture;
uniform float time;
uniform vec2 resolution;
uniform vec3 mouse;

// https://www.shadertoy.com/view/XtlSD7

void main()
{
    float t = time + 5.;
	float z = 6.;

	const int n = 100; // particle count
    
    vec3 startColor = vec3(0,0.64,0.2);
	vec3 endColor = vec3(0.06,0.35,0.85);
    
	float startRadius = 0.84;
	float endRadius = 1.6;
    
	float power = 0.51;
	float duration = 4.;
    
	vec2 s = resolution;
	vec2 v = z*(2.*uv.xy-s)/s.y;
    
    // Mouse axis y => zoom
	if(mouse.z>0.) v *= mouse.y/s.y * 20.;
    
    // Mouse axis x => duration
	if(mouse.z>0.) duration = mouse.x/s.x * 10.;
    
	vec3 col = vec3(0.,0.,1.);
    
	vec2 pm = v.yx*2.8;
    
	float dMax = duration;
    

    float evo = (sin(time*.01+400.)*.5+.5)*99.+1.;
	
	float mb = 0.;
	float mbRadius = 0.;
	float sum = 0.;
	for(int i=0;i<n;i++)
	{
		float d = fract(t*power+48934.4238*sin(float(i/int(evo))*692.7398));
    	 		
		float tt = 0.;
			
        float a = 6.28*float(i)/float(n);

        float x = d*cos(a)*duration;

        float y = d*sin(a)*duration;
        
		float distRatio = d/dMax;
        
		mbRadius = mix(startRadius, endRadius, distRatio); 
        
		vec2 p = v - vec2(x,y);//*vec2(1,sin(a+3.14159/2.));
        
		mb = mbRadius/dot(p,p);
    	
		sum += mb;
        
		col = mix(col, mix(startColor, endColor, distRatio), mb/sum);
	}
    
	sum /= float(n);
    
	col = normalize(col) * sum;
    
	sum = clamp(sum, 0., .4);
    
	vec3 tex = vec3(1.);
     
	col *= smoothstep(tex, vec3(0.), vec3(sum));
        
	gl_FragColor.rgb = col;
}