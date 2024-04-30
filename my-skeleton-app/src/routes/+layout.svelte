<script lang="ts">
  import "../app.postcss";
  import { Navigation } from "@sveltejs/kit";

  // Highlight JS
  import hljs from "highlight.js/lib/core";
  import "highlight.js/styles/github-dark.css";
  import {
    AppBar,
    AppRailAnchor,
    storeHighlightJs,
  } from "@skeletonlabs/skeleton";
  import xml from "highlight.js/lib/languages/xml"; // for HTML
  import css from "highlight.js/lib/languages/css";
  import javascript from "highlight.js/lib/languages/javascript";
  import typescript from "highlight.js/lib/languages/typescript";

  hljs.registerLanguage("xml", xml); // for HTML
  hljs.registerLanguage("css", css);
  hljs.registerLanguage("javascript", javascript);
  hljs.registerLanguage("typescript", typescript);
  storeHighlightJs.set(hljs);

  // Floating UI for Popups
  import {
    computePosition,
    autoUpdate,
    flip,
    shift,
    offset,
    arrow,
  } from "@floating-ui/dom";
  import { storePopup } from "@skeletonlabs/skeleton";
  storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });
</script>

<div class="flex">
  <a class="btn" href="/deployContracts">Developer DAO | D_D</a>
  <a class="btn ml-24 bg-slate-950 rounded-lg" href="/deployContracts"
    >Deploy Contracts</a
  >
  <a class="btn ml-6 bg-slate-950 rounded-lg" href="/config">Config</a>
  <a class="btn ml-6 bg-slate-950 rounded-lg" href="/deployments">Deployments</a
  >
  <a class="btn ml-6 bg-slate-950 rounded-lg" href="/generateKeyStore"
    >Key Store</a
  >
</div>

<slot />
