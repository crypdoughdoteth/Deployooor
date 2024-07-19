

export const index = 3;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/config/_page.svelte.js')).default;
export const imports = ["_app/immutable/nodes/3.DMSKvacA.js","_app/immutable/chunks/scheduler.Z0cX8yaB.js","_app/immutable/chunks/index.DUYAcDti.js"];
export const stylesheets = [];
export const fonts = [];
