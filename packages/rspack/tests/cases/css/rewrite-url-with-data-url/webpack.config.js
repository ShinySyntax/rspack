module.exports = {
	output: {
		cssFilename: "css/[name].css"
	},
	module: {
		rules: [
			{
				test: /\.png$/i,
				type: "asset",
				parser: {
					dataUrlCondition: {
						maxSize: 30000
					}
				},
				generator: {
					filename: "image/[name].[contenthash:8][ext]"
				}
			}
		]
	}
};
