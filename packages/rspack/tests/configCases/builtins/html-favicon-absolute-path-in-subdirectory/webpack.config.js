const path = require("path");

module.exports = {
	builtins: {
		html: [
			{
				publicPath: "/",
				favicon: path.resolve(__dirname, "./static/favicon.ico")
			}
		]
	}
};
