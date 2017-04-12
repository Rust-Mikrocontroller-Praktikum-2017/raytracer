

function trigger_browser_download(blob, fileName) {
	var url = window.URL.createObjectURL(blob);
	var a = document.createElement("a");
	a.href = url;
	a.download = fileName;
	a.click();

	// needed for firefox
	setTimeout(function() {
		window.URL.revokeObjectURL(url);
	}, 1000);
}

function url_get_image_data(url, cb) {
	var canvas = document.createElement('canvas'),
	    context = canvas.getContext('2d'),
	    image = new Image();

	image.addEventListener('load', function() {
		canvas.width = image.width;
		canvas.height = image.height;
		context.drawImage(image, 0, 0, canvas.width, canvas.height);
		cb(context.getImageData(0, 0, canvas.width, canvas.height));
	}, false);
	image.src = url;
}

function imagedata_to_rgb_blob(img) {
	var rgb = new Uint8Array(img.width*img.height*3);
	for(var x = 0; x < img.width; ++x) {
		for(var y = 0; y < img.height; ++y) {
			var i = y * img.width + x;
			rgb[i*3] = img.data[i*4];
			rgb[(i*3)+1] = img.data[(i*4)+1];
			rgb[(i*3)+2] = img.data[(i*4)+2];
		}
	}

	return new Blob([rgb], {type: "application/octet-stream"});
}

url_get_image_data("./texture.jpg", function(imgdata) {
	var blob = imagedata_to_rgb_blob(imgdata);
	trigger_browser_download(blob, "texture.rgb");
});
