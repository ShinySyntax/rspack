module.exports = {
	module: {
		rules: [
			{
				test: "\\.s[ac]ss$",
				uses: [
					{
						builtinLoader: "sass-loader",
						options: {
							additionalData: "$prepended-data: hotpink;"
						}
					}
				],
				type: "css"
			}
		]
	}
};
