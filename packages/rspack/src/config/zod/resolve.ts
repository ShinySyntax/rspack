import { z } from "zod";

const resolveAlias = z.record(
	z
		.literal(false)
		.or(z.string())
		.or(z.array(z.literal(false).or(z.string())))
);

const baseResolve = z.strictObject({
	alias: resolveAlias.optional(),
	browserField: z.boolean().optional(),
	conditionNames: z.string().array().optional(),
	extensions: z.string().array().optional(),
	fallback: resolveAlias.optional(),
	mainFields: z.string().array().optional(),
	mainFiles: z.string().array().optional(),
	modules: z.string().array().optional(),
	preferRelative: z.boolean().optional(),
	tsConfigPath: z.string().optional(),
	fullySpecified: z.boolean().optional(),
	exportsFields: z.string().array().optional(),
	extensionAlias: z.record(z.string().or(z.string().array())).optional()
});

// Recursive types need to write this way.
// See https://zod.dev/?id=recursive-types
type ResolveConfig = z.infer<typeof baseResolve> & {
	byDependency?: Record<string, ResolveConfig>;
};

export function resolve(): z.ZodType<ResolveConfig> {
	return baseResolve.extend({
		byDependency: z.lazy(() => z.record(resolve())).optional()
	});
}
