/// <reference types="svelte" />

declare namespace svelte.JSX {
  interface DOMAttributes<T extends EventTarget> {
    onclickOutside?: TouchEventHandler<T> | undefined | null;
  }
}
