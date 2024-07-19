import { s as setContext, c as create_ssr_component, a as subscribe, b as createEventDispatcher, e as escape, d as add_attribute, f as spread, g as escape_object, v as validate_component, m as missing_component } from "../../chunks/ssr.js";
import { w as writable } from "../../chunks/index.js";
import { i as initializeModalStore, p as prefersReducedMotionStore, g as getModalStore, f as fly } from "../../chunks/ProgressBar.svelte_svelte_type_style_lang.js";
import hljs from "highlight.js/lib/core";
import xml from "highlight.js/lib/languages/xml";
import css from "highlight.js/lib/languages/css";
import javascript from "highlight.js/lib/languages/javascript";
import typescript from "highlight.js/lib/languages/typescript";
import { computePosition, autoUpdate, flip, shift, offset, arrow } from "@floating-ui/dom";
const app = "";
const storeHighlightJs = writable(void 0);
const storePopup = writable(void 0);
const DRAWER_STORE_KEY = "drawerStore";
function initializeDrawerStore() {
  const drawerStore = drawerService();
  return setContext(DRAWER_STORE_KEY, drawerStore);
}
function drawerService() {
  const { subscribe: subscribe2, set, update } = writable({});
  return {
    subscribe: subscribe2,
    set,
    update,
    /** Open the drawer. */
    open: (newSettings) => update(() => {
      return { open: true, ...newSettings };
    }),
    /** Close the drawer. */
    close: () => update((d) => {
      d.open = false;
      return d;
    })
  };
}
const toastDefaults = { message: "Missing Toast Message", autohide: true, timeout: 5e3 };
const TOAST_STORE_KEY = "toastStore";
function initializeToastStore() {
  const toastStore = toastService();
  return setContext(TOAST_STORE_KEY, toastStore);
}
function randomUUID() {
  const random = Math.random();
  return Number(random).toString(32);
}
function toastService() {
  const { subscribe: subscribe2, set, update } = writable([]);
  const close = (id) => update((tStore) => {
    if (tStore.length > 0) {
      const index = tStore.findIndex((t) => t.id === id);
      const selectedToast = tStore[index];
      if (selectedToast) {
        if (selectedToast.callback)
          selectedToast.callback({ id, status: "closed" });
        if (selectedToast.timeoutId)
          clearTimeout(selectedToast.timeoutId);
        tStore.splice(index, 1);
      }
    }
    return tStore;
  });
  function handleAutoHide(toast) {
    if (toast.autohide === true) {
      return setTimeout(() => {
        close(toast.id);
      }, toast.timeout);
    }
  }
  return {
    subscribe: subscribe2,
    close,
    /** Add a new toast to the queue. */
    trigger: (toast) => {
      const id = randomUUID();
      update((tStore) => {
        if (toast && toast.callback)
          toast.callback({ id, status: "queued" });
        if (toast.hideDismiss)
          toast.autohide = true;
        const tMerged = { ...toastDefaults, ...toast, id };
        tMerged.timeoutId = handleAutoHide(tMerged);
        tStore.push(tMerged);
        return tStore;
      });
      return id;
    },
    /** Remain visible on hover */
    freeze: (index) => update((tStore) => {
      if (tStore.length > 0)
        clearTimeout(tStore[index].timeoutId);
      return tStore;
    }),
    /** Cancel remain visible on leave */
    unfreeze: (index) => update((tStore) => {
      if (tStore.length > 0)
        tStore[index].timeoutId = handleAutoHide(tStore[index]);
      return tStore;
    }),
    /** Remove all toasts from queue */
    clear: () => set([])
  };
}
function initializeStores() {
  initializeModalStore();
  initializeToastStore();
  initializeDrawerStore();
}
const cBackdrop = "fixed top-0 left-0 right-0 bottom-0 bg-surface-backdrop-token p-4";
const cTransitionLayer = "w-full h-fit min-h-full overflow-y-auto flex justify-center";
const cModal = "block overflow-y-auto";
const cModalImage = "w-full h-auto";
const Modal = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let cPosition;
  let classesBackdrop;
  let classesTransitionLayer;
  let classesModal;
  let parent;
  let $modalStore, $$unsubscribe_modalStore;
  let $prefersReducedMotionStore, $$unsubscribe_prefersReducedMotionStore;
  $$unsubscribe_prefersReducedMotionStore = subscribe(prefersReducedMotionStore, (value) => $prefersReducedMotionStore = value);
  createEventDispatcher();
  let { components = {} } = $$props;
  let { position = "items-center" } = $$props;
  let { background = "bg-surface-100-800-token" } = $$props;
  let { width = "w-modal" } = $$props;
  let { height = "h-auto" } = $$props;
  let { padding = "p-4" } = $$props;
  let { spacing = "space-y-4" } = $$props;
  let { rounded = "rounded-container-token" } = $$props;
  let { shadow = "shadow-xl" } = $$props;
  let { zIndex = "z-[999]" } = $$props;
  let { buttonNeutral = "variant-ghost-surface" } = $$props;
  let { buttonPositive = "variant-filled" } = $$props;
  let { buttonTextCancel = "Cancel" } = $$props;
  let { buttonTextConfirm = "Confirm" } = $$props;
  let { buttonTextSubmit = "Submit" } = $$props;
  let { regionBackdrop = "" } = $$props;
  let { regionHeader = "text-2xl font-bold" } = $$props;
  let { regionBody = "max-h-[200px] overflow-hidden" } = $$props;
  let { regionFooter = "flex justify-end space-x-2" } = $$props;
  let { transitions = !$prefersReducedMotionStore } = $$props;
  let { transitionIn = fly } = $$props;
  let { transitionInParams = { duration: 150, opacity: 0, x: 0, y: 100 } } = $$props;
  let { transitionOut = fly } = $$props;
  let { transitionOutParams = { duration: 150, opacity: 0, x: 0, y: 100 } } = $$props;
  let promptValue;
  const buttonTextDefaults = {
    buttonTextCancel,
    buttonTextConfirm,
    buttonTextSubmit
  };
  let currentComponent;
  let modalElement;
  let windowHeight;
  let backdropOverflow = "overflow-y-hidden";
  const modalStore = getModalStore();
  $$unsubscribe_modalStore = subscribe(modalStore, (value) => $modalStore = value);
  function handleModals(modals) {
    if (modals[0].type === "prompt")
      promptValue = modals[0].value;
    buttonTextCancel = modals[0].buttonTextCancel || buttonTextDefaults.buttonTextCancel;
    buttonTextConfirm = modals[0].buttonTextConfirm || buttonTextDefaults.buttonTextConfirm;
    buttonTextSubmit = modals[0].buttonTextSubmit || buttonTextDefaults.buttonTextSubmit;
    currentComponent = typeof modals[0].component === "string" ? components[modals[0].component] : modals[0].component;
  }
  function onModalHeightChange(modal) {
    let modalHeight = modal?.clientHeight;
    if (!modalHeight)
      modalHeight = modal?.firstChild?.clientHeight;
    if (!modalHeight)
      return;
    if (modalHeight > windowHeight) {
      backdropOverflow = "overflow-y-auto";
    } else {
      backdropOverflow = "overflow-y-hidden";
    }
  }
  function onClose() {
    if ($modalStore[0].response)
      $modalStore[0].response(false);
    modalStore.close();
  }
  if ($$props.components === void 0 && $$bindings.components && components !== void 0)
    $$bindings.components(components);
  if ($$props.position === void 0 && $$bindings.position && position !== void 0)
    $$bindings.position(position);
  if ($$props.background === void 0 && $$bindings.background && background !== void 0)
    $$bindings.background(background);
  if ($$props.width === void 0 && $$bindings.width && width !== void 0)
    $$bindings.width(width);
  if ($$props.height === void 0 && $$bindings.height && height !== void 0)
    $$bindings.height(height);
  if ($$props.padding === void 0 && $$bindings.padding && padding !== void 0)
    $$bindings.padding(padding);
  if ($$props.spacing === void 0 && $$bindings.spacing && spacing !== void 0)
    $$bindings.spacing(spacing);
  if ($$props.rounded === void 0 && $$bindings.rounded && rounded !== void 0)
    $$bindings.rounded(rounded);
  if ($$props.shadow === void 0 && $$bindings.shadow && shadow !== void 0)
    $$bindings.shadow(shadow);
  if ($$props.zIndex === void 0 && $$bindings.zIndex && zIndex !== void 0)
    $$bindings.zIndex(zIndex);
  if ($$props.buttonNeutral === void 0 && $$bindings.buttonNeutral && buttonNeutral !== void 0)
    $$bindings.buttonNeutral(buttonNeutral);
  if ($$props.buttonPositive === void 0 && $$bindings.buttonPositive && buttonPositive !== void 0)
    $$bindings.buttonPositive(buttonPositive);
  if ($$props.buttonTextCancel === void 0 && $$bindings.buttonTextCancel && buttonTextCancel !== void 0)
    $$bindings.buttonTextCancel(buttonTextCancel);
  if ($$props.buttonTextConfirm === void 0 && $$bindings.buttonTextConfirm && buttonTextConfirm !== void 0)
    $$bindings.buttonTextConfirm(buttonTextConfirm);
  if ($$props.buttonTextSubmit === void 0 && $$bindings.buttonTextSubmit && buttonTextSubmit !== void 0)
    $$bindings.buttonTextSubmit(buttonTextSubmit);
  if ($$props.regionBackdrop === void 0 && $$bindings.regionBackdrop && regionBackdrop !== void 0)
    $$bindings.regionBackdrop(regionBackdrop);
  if ($$props.regionHeader === void 0 && $$bindings.regionHeader && regionHeader !== void 0)
    $$bindings.regionHeader(regionHeader);
  if ($$props.regionBody === void 0 && $$bindings.regionBody && regionBody !== void 0)
    $$bindings.regionBody(regionBody);
  if ($$props.regionFooter === void 0 && $$bindings.regionFooter && regionFooter !== void 0)
    $$bindings.regionFooter(regionFooter);
  if ($$props.transitions === void 0 && $$bindings.transitions && transitions !== void 0)
    $$bindings.transitions(transitions);
  if ($$props.transitionIn === void 0 && $$bindings.transitionIn && transitionIn !== void 0)
    $$bindings.transitionIn(transitionIn);
  if ($$props.transitionInParams === void 0 && $$bindings.transitionInParams && transitionInParams !== void 0)
    $$bindings.transitionInParams(transitionInParams);
  if ($$props.transitionOut === void 0 && $$bindings.transitionOut && transitionOut !== void 0)
    $$bindings.transitionOut(transitionOut);
  if ($$props.transitionOutParams === void 0 && $$bindings.transitionOutParams && transitionOutParams !== void 0)
    $$bindings.transitionOutParams(transitionOutParams);
  {
    if ($modalStore.length)
      handleModals($modalStore);
  }
  {
    onModalHeightChange(modalElement);
  }
  cPosition = $modalStore[0]?.position ?? position;
  classesBackdrop = `${cBackdrop} ${regionBackdrop} ${zIndex} ${$$props.class ?? ""} ${$modalStore[0]?.backdropClasses ?? ""}`;
  classesTransitionLayer = `${cTransitionLayer} ${cPosition ?? ""}`;
  classesModal = `${cModal} ${background} ${width} ${height} ${padding} ${spacing} ${rounded} ${shadow} ${$modalStore[0]?.modalClasses ?? ""}`;
  parent = {
    position,
    // ---
    background,
    width,
    height,
    padding,
    spacing,
    rounded,
    shadow,
    // ---
    buttonNeutral,
    buttonPositive,
    buttonTextCancel,
    buttonTextConfirm,
    buttonTextSubmit,
    // ---
    regionBackdrop,
    regionHeader,
    regionBody,
    regionFooter,
    // ---
    onClose
  };
  $$unsubscribe_modalStore();
  $$unsubscribe_prefersReducedMotionStore();
  return ` ${$modalStore.length > 0 ? `   <div class="${"modal-backdrop " + escape(classesBackdrop, true) + " " + escape(backdropOverflow, true)}" data-testid="modal-backdrop"> <div class="${"modal-transition " + escape(classesTransitionLayer, true)}">${$modalStore[0].type !== "component" ? ` <div class="${"modal " + escape(classesModal, true)}" data-testid="modal" role="dialog" aria-modal="true"${add_attribute("aria-label", $modalStore[0].title ?? "", 0)}${add_attribute("this", modalElement, 0)}> ${$modalStore[0]?.title ? `<header class="${"modal-header " + escape(regionHeader, true)}"><!-- HTML_TAG_START -->${$modalStore[0].title}<!-- HTML_TAG_END --></header>` : ``}  ${$modalStore[0]?.body ? `<article class="${"modal-body " + escape(regionBody, true)}"><!-- HTML_TAG_START -->${$modalStore[0].body}<!-- HTML_TAG_END --></article>` : ``}  ${$modalStore[0]?.image && typeof $modalStore[0]?.image === "string" ? `<img class="${"modal-image " + escape(cModalImage, true)}"${add_attribute("src", $modalStore[0]?.image, 0)} alt="Modal">` : ``}  ${$modalStore[0].type === "alert" ? ` <footer class="${"modal-footer " + escape(regionFooter, true)}"><button type="button" class="${"btn " + escape(buttonNeutral, true)}">${escape(buttonTextCancel)}</button></footer>` : `${$modalStore[0].type === "confirm" ? ` <footer class="${"modal-footer " + escape(regionFooter, true)}"><button type="button" class="${"btn " + escape(buttonNeutral, true)}">${escape(buttonTextCancel)}</button> <button type="button" class="${"btn " + escape(buttonPositive, true)}">${escape(buttonTextConfirm)}</button></footer>` : `${$modalStore[0].type === "prompt" ? ` <form class="space-y-4"><input${spread(
    [
      { class: "modal-prompt-input input" },
      { name: "prompt" },
      { type: "text" },
      escape_object($modalStore[0].valueAttr)
    ],
    {}
  )}${add_attribute("value", promptValue, 0)}> <footer class="${"modal-footer " + escape(regionFooter, true)}"><button type="button" class="${"btn " + escape(buttonNeutral, true)}">${escape(buttonTextCancel)}</button> <button type="submit" class="${"btn " + escape(buttonPositive, true)}">${escape(buttonTextSubmit)}</button></footer></form>` : ``}`}`}</div>` : `  <div class="${"modal contents " + escape($modalStore[0]?.modalClasses ?? "", true)}" data-testid="modal-component" role="dialog" aria-modal="true"${add_attribute("aria-label", $modalStore[0].title ?? "", 0)}${add_attribute("this", modalElement, 0)}>${currentComponent?.slot ? `${validate_component(currentComponent?.ref || missing_component, "svelte:component").$$render($$result, Object.assign({}, currentComponent?.props, { parent }), {}, {
    default: () => {
      return `<!-- HTML_TAG_START -->${currentComponent?.slot}<!-- HTML_TAG_END -->`;
    }
  })}` : `${validate_component(currentComponent?.ref || missing_component, "svelte:component").$$render($$result, Object.assign({}, currentComponent?.props, { parent }), {}, {})}`}</div>`}</div></div>` : ``}`;
});
const githubDark = "";
const Layout = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  initializeStores();
  hljs.registerLanguage("xml", xml);
  hljs.registerLanguage("css", css);
  hljs.registerLanguage("javascript", javascript);
  hljs.registerLanguage("typescript", typescript);
  storeHighlightJs.set(hljs);
  storePopup.set({
    computePosition,
    autoUpdate,
    flip,
    shift,
    offset,
    arrow
  });
  initializeStores();
  return `${validate_component(Modal, "Modal").$$render($$result, {}, {}, {})} <div class="flex pt-2 pb-2 bg-slate-950/50" data-svelte-h="svelte-9m4x9t"><a class="btn text-xl" href="/deployContracts">Deploooyer | D_D</a> <a class="btn ml-24 bg-slate-950 rounded-lg" href="/deployContracts">Deploy Contracts</a> <a class="btn ml-6 bg-slate-950 rounded-lg" href="/config">Config</a> <a class="btn ml-6 bg-slate-950 rounded-lg" href="/deployments">Deployments</a> <a class="btn ml-6 bg-slate-950 rounded-lg" href="/generateKeyStore">Key Store</a></div> ${slots.default ? slots.default({}) : ``}`;
});
export {
  Layout as default
};
