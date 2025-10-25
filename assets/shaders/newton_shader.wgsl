struct Root {
    pos: vec2f,
    color: vec4f,
};

struct Params {
    epsilon: f32,
    max_iter: i32,
    scale: f32,
    offset: vec2f,
    aspect_ratio: f32,
}

@group(2) @binding(0) var<storage> roots: array<Root>;
@group(2) @binding(1) var<storage> coeff: array<vec2f>;
@group(2) @binding(2) var<storage> deriv: array<vec2f>;
@group(2) @binding(3) var<uniform> params: Params;

@fragment
fn fragment(
    @location(2) uv: vec2f,
    ) -> @location(0) vec4f {

    // Convert uv space to coordinate space
    let coord = ((uv * 2.0 - 1.0) * vec2f(1.0, -1.0)) / params.scale * vec2f(params.aspect_ratio, 1.0) - params.offset;

    // Use Newton-Raphson method to approximate the root
    let nr = newton_raphson(coord);
    let approx_root = nr.x;

    // Find the closest real root
    var closest_root = roots[0];
    var closest_dist = distance(closest_root.pos, approx_root);
    
    let array_length = i32(arrayLength(&roots));
    for (var i = 1; i < array_length; i++) {
        let root = roots[i];
        let dist = distance(root.pos, approx_root);
        if dist < closest_dist {
            closest_root = root;
            closest_dist = dist;
        }
    }

    // let mul = 1.0 - (f32(nr.i) / f32(params.max_iter));
    return closest_root.color;
}

struct nr_return {
    x: vec2f,
    i: i32,
};

fn newton_raphson(x0: vec2f) -> nr_return {
    let coeff_len = i32(arrayLength(&coeff));
    let deriv_len = coeff_len - 1;

    var x = x0;
    var h = complex_div(eval_polynomial(x, coeff_len), eval_derivative(x, deriv_len));
    var i = 0;
    while length(h) >= params.epsilon && i < params.max_iter {
        h = complex_div(eval_polynomial(x, coeff_len), eval_derivative(x, deriv_len));
        x -= h;
        i += 1;
    }
    return nr_return(x, i);
}

fn eval_polynomial(x: vec2f, len: i32) -> vec2f {
    var result = vec2f(0.0, 0.0);
    var power = vec2f(1.0, 0.0); // x^0 = 1

    for (var i = 0; i < len; i++) {
        let c = coeff[i];
        result += complex_mul(c, power);
        power = complex_mul(power, x);
    }

    return result;
}

fn eval_derivative(x: vec2f, len: i32) -> vec2f {
    var result = vec2f(0.0, 0.0);
    var power = vec2f(1.0, 0.0); // x^0 = 1

    for (var i = 0; i < len; i++) {
        let c = deriv[i];
        result += complex_mul(c, power);
        power = complex_mul(power, x);
    }

    return result;
}

fn complex_mul(a: vec2f, b: vec2f) -> vec2f {
    let real = a.x * b.x - a.y * b.y;
    let imag = a.x * b.y + a.y * b.x;
    return vec2f(real, imag);
}

fn complex_div(a: vec2f, b: vec2f) -> vec2f {
    let b_squared = b.x * b.x + b.y * b.y;
    let inv_denominator = 1.0 / b_squared;
    let real = (a.x * b.x + a.y * b.y) * inv_denominator;
    let imag = (a.y * b.x - a.x * b.y) * inv_denominator;
    return vec2f(real, imag);
}