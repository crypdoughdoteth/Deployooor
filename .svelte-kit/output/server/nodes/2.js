

export const index = 2;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/_page.svelte.js')).default;
export const imports = ["_app/immutable/nodes/2.D2qiWs0t.js","_app/immutable/chunks/scheduler.Z0cX8yaB.js","_app/immutable/chunks/index.DUYAcDti.js"];
export const stylesheets = [];
export const fonts = [];
