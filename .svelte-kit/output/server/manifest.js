export const manifest = (() => {
function __memo(fn) {
	let value;
	return () => value ??= (value = fn());
}

return {
	appDir: "_app",
	appPath: "_app",
	assets: new Set([]),
	mimeTypes: {},
	_: {
		client: {"start":"_app/immutable/entry/start.DoWUzrIn.js","app":"_app/immutable/entry/app.DWWj76B7.js","imports":["_app/immutable/entry/start.DoWUzrIn.js","_app/immutable/chunks/entry.CPmCPlOH.js","_app/immutable/chunks/scheduler.Z0cX8yaB.js","_app/immutable/chunks/index.XjgnMDCI.js","_app/immutable/entry/app.DWWj76B7.js","_app/immutable/chunks/scheduler.Z0cX8yaB.js","_app/immutable/chunks/index.DUYAcDti.js"],"stylesheets":[],"fonts":[],"uses_env_dynamic_public":false},
		nodes: [
			__memo(() => import('./nodes/0.js')),
			__memo(() => import('./nodes/1.js')),
			__memo(() => import('./nodes/2.js')),
			__memo(() => import('./nodes/3.js')),
			__memo(() => import('./nodes/4.js')),
			__memo(() => import('./nodes/5.js')),
			__memo(() => import('./nodes/6.js'))
		],
		routes: [
			{
				id: "/",
				pattern: /^\/$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 2 },
				endpoint: null
			},
			{
				id: "/config",
				pattern: /^\/config\/?$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 3 },
				endpoint: null
			},
			{
				id: "/deployContracts",
				pattern: /^\/deployContracts\/?$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 4 },
				endpoint: null
			},
			{
				id: "/deployments",
				pattern: /^\/deployments\/?$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 5 },
				endpoint: null
			},
			{
				id: "/generateKeyStore",
				pattern: /^\/generateKeyStore\/?$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 6 },
				endpoint: null
			}
		],
		matchers: async () => {
			
			return {  };
		},
		server_assets: {}
	}
}
})();
