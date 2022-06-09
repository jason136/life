/** @constructor */
function Renderer() {
    var 
        /** @type {number} */
        canvas_offset_x = 0,
        /** @type {number} */
        canvas_offset_y = 0,

        canvas_width,
        canvas_height,

        canvas,
        context,

        image_data,
        image_data_data,

        border_width,
        cell_color_rgb,

        renderer = this;

    var pixel_ratio = 1;

    this.cell_color = null;
    this.background_color = null;

    this.border_width = 0;

    this.init = (dom_parent) => {
        canvas = document.getElementById("canvas");

        if (!canvas.getContext) {
            return false;
        }

        renderer.canvas = canvas;
        context = canvas.getContext("2d");

        dom_parent.appendChild(canvas);
        return true;
    }

    this.set_size = (width, height) => {
        if(width !== canvas_width || height !== canvas_height){
            canvas.style.width = width + "px";
            canvas.style.height = height + "px";

            pixel_ratio = window.devicePixelRatio;

            canvas.width = Math.round(width * pixel_ratio);
            canvas.height = Math.round(height * pixel_ratio);

            canvas_width = canvas.width;
            canvas_height = canvas.height;

            image_data = context.createImageData(canvas_width, canvas_height);
            image_data_data = new Int32Array(image_data.data.buffer);

            for(var i = 0; i < width * height; i++) {
                image_data_data[i] = 0xFF << 24;
            }
        }
    }

    this.draw_node = (node, size, left, top) => {
        if (left + size + canvas_offset_x < 0 ||
            top + size + canvas_offset_y < 0 ||
            left + canvas_offset_x >= canvas_width ||
            top + canvas_offset_y >= canvas_height
        ) return;

        if (size <= 1) {
            if(node.population) fill_square(left + canvas_offset_x | 0, top + canvas_offset_y | 0, 1);
        }
        else if (node.level === 0) {
            if(node.population) fill_square(left + canvas_offset_x, top + canvas_offset_y, cell_width);
        }
        else {
            size /= 2;

            draw_node(node.nw, size, left, top);
            draw_node(node.ne, size, left + size, top);
            draw_node(node.sw, size, left, top + size);
            draw_node(node.se, size, left + size, top + size);
        }
    }

    this.fill_square = (x, y, size) => {
        var width = size - border_width, 
            height = width;

        if (x < 0) {
            width += x;
            x = 0;
        }
        if (x + width > canvas_width) {
            width = canvas_width - x;
        }

        if (y < 0) {
            height += y;
            y = 0;
        }
        if (y + height > canvas_height) {
            height = canvas_height - y;
        }

        if (width <= 0 || height <= 0) return;

        var pointer = x + y * canvas_width, 
            row_width = canvas_width - width;

        var color = cell_color_rgb.r | cell_color_rgb.g << 8 | cell_color_rgb.b << 16 | 0xFF << 24;

        for (var i = 0; i < height; i++) {
            for (var j = 0; j < width; j++) {
                image_data_data[pointer] = color;
                pointer++;
            }
            pointer += row_width;
        }
    }

    this.redraw = () => {
        var bg_color_rgb = color2rgb(background_color);
        var bg_color_int = bg_color_rgb.r | bg_color_rgb.g << 8 | bg_color_rgb.b << 16 | 0xFF << 24;

        border_width = renderer.border_width * renderer.cell_width | 0;
        cell_color_rgb = color2rgb(renderer.cell_color);

        var count = canvas_width * canvas_height;

        for (var i = 0; i < count; i++) {
            image_data_data[i] = bg_color_int;
        }

        var size = Math.pow(2, node.level - 1) * renderer.cell_width;

        draw_node(node, 2 * size, -size, -size);

        context.putImageData(image_data, 0, 0);
    }

    this.zoom = (out, center_x, center_y) => {
        if (out) {
            canvas_offset_x -= Math.round((canvas_width / 2 - center_x) * 0.5);
            canvas_offset_y -= Math.round((canvas_height / 2 - center_y) * 0.5);
            renderer.cell_width *= 0.5;
        }
        else {
            canvas_offset_x += Math.round(canvas_offset_x - center_x);
            canvas_offset_y += Math.round(canvas_offset_y - center_y);
            renderer.cell_width *= 2;
        }
    }

    this.zoom_at = (out, center_x, center_y) => {
        zoom(out, center_x * pixel_ratio, center_y * pixel_ratio);
    }
    this.zoom_centered = (out) => {
        zoom(out, canvas_width >> 1, canvas_height >> 1);
    }
    this.zoom_to = (level) => {
        while(renderer.cell_width > level) {
            zoom_centered(true);
        }
        while(renderer.cell_width * 2 < level) {
            zoom_centered(false);
        }
    }

    this.center_view = () => {
        canvas_offset_x = canvas_width >> 1;
        canvas_offset_y = canvas_height >> 1;
    }
    this.move = (dx, dy) => {
        convas_offset_x += Math.round(dx * pixel_ratio);
        canvas_offset_y += Math.round(dy * pixel_ratio);
    }

    this.fit_bounds = (bounds) => {
        var width = bounds.right - bounds.left,
            height = bounds.bottom - bounds.top,
            relative_size, 
            x, y;

        if (isFinite(width) && isFinite(height)) {
            relative_size = Math.max(16, canvas_width / width, canvas_height / height);
            zoom_to(relative_size);

            x = Math.round(canvas_width / 2 - (bounds.left + width / 2) * drawer.cell_width);
            y = Math.round(canvas_height / 2 - (bounds.top + height / 2) * drawer.cell_width);
        }
        else {
            zoom_to(16);
            x = canvas_width >> 1;
            y = canvas_height >> 1;
        }

        canvas_offset_x = x;
        canvas_offset_y = y;
    }

    this.draw_cell = (x, y, set) => {
        var cell_x = x * drawer.cell_width + canvas_offset_x,
            cell_y = y * drawer.cell_width + canvas_offset_y,
            width = Math.ceil(drawer.cell_width) -
                (drawer.cell_width * drawer.border_width | 0);

        if(set) {
            context.fillStyle = drawer.cell_color;
        }
        else {
            context.fillStyle = drawer.background_color;
        }

        context.fillRect(cell_x, cell_y, width, width);
    }

    this.pixel2cell = (x, y) => {
        return {
            x : Math.floor((x * pixel_ratio - canvas_offset_x + drawer.border_width / 2) / drawer.cell_width),
            y : Math.floor((y * pixel_ratio - canvas_offset_y + drawer.border_width / 2) / drawer.cell_width)
        };
    }

    this.color2rgb = (color) => {
        if (color.length === 4) {
            return {
                r: parseInt(color[1] + color[1], 16),
                g: parseInt(color[2] + color[2], 16),
                b: parseInt(color[3] + color[3], 16)
            };
        }
        else {
            return {
                r: parseInt(color.slice(1, 3), 16),
                g: parseInt(color.slice(3, 5), 16),
                b: parseInt(color.slice(5, 7), 16)
            };
        }
    }
}

export { Renderer };

// https://github.com/copy/life/blob/924c90afb529ad5d417f11d795bc1b400fff8d18/draw.js