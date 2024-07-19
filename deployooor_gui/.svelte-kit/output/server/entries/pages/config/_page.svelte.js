import { c as create_ssr_component } from "../../../chunks/ssr.js";
const Page = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  return `<form class="w-1/3 m-auto mt-24" data-svelte-h="svelte-1fwmgzj"><label class="label"><span>Provider URL</span> <input class="input" type="text"></label> <label class="label mt-4"><span>Etherscan API Key</span> <input class="input" type="text"></label> <button type="button" class="btn rounded-lg bg-slate-950 mt-4">Set Config</button></form>`;
});
export {
  Page as default
};
