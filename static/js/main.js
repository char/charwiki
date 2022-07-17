const ctx = document.body.dataset;

import { q, t, h, r } from "./brief-html.js";

r(() => {
  const main = q("#article-content");

  const edit = async () => {
    if (q("#article-editor") != null) {
      q("#article-editor").remove();
      main.style.display = null;
      return;
    }

    const url = "/api/article/" + ctx.articleSlug;
    const article = await fetch(url).then((r) => r.json());
    if (article.error != null) {
      alert(article.error);
      return;
    }

    const saveButton = h("button", { id: "save-button" }, "Save");
    const cancelButton = h("button", { id: "save-button" }, "Cancel");

    // TODO: Actions for save & cancel

    const editor = h("main", { id: "article-editor" }, [
      h("h2", {}, ["Editing: ", h("code", {}, article.path)]),
      h(
        "input",
        {
          id: "article-title-field",
          value: article.title,
          placeholder: "Title",
        },
        []
      ),
      h("textarea", { id: "article-content-field", value: article.source }, []),
      h("section", {}, [saveButton, cancelButton]),
    ]);

    main.parentNode.insertBefore(editor, main);
    main.style.display = "none";
  };

  main.addEventListener("mousedown", () => {
    main.addEventListener("dblclick", edit);
    setTimeout(() => {
      main.removeEventListener("dblclick", edit);
    }, 350);
  });
});

window.onbeforeunload = function () {
  if (q("#article-editor") != null) {
    return "";
  }

  return null;
};
