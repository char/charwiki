export const q = (selector) => document.querySelector(selector);
export const t = (text) => document.createTextNode(text);
export const h = (tag, attributes, children) => {
  if (!Array.isArray(children)) {
    children = [children];
  }

  children = children.map((x) => {
    if (x instanceof HTMLElement) return x;
    return t(x);
  });

  const elem = document.createElement(tag);
  Object.assign(elem, attributes);
  elem.append(...children);
  return elem;
};
export const r = (f) => {
  if (document.readyState === "complete") {
    f();
  } else {
    document.addEventListener("DOMContentLoaded", f);
  }
};
