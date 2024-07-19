

export const index = 5;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/deployments/_page.svelte.js')).default;
export const imports = ["_app/immutable/nodes/5.BqD4OG76.js","_app/immutable/chunks/scheduler.Z0cX8yaB.js","_app/immutable/chunks/index.DUYAcDti.js"];
export const stylesheets = [];
export const fonts = [];
