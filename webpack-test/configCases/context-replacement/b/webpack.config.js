var webpack = require("../../../../");

/** @type {import("@rspack/core").Configuration} */
module.exports = {
	plugins: [
		new webpack.ContextReplacementPlugin(/context-replacement.b$/, /^\.\/only/)
	]
};
