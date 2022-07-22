const ctx = document.body.dataset;

import { q, t, h, r } from "./brief-html.js";

r(() => {
  const main = q("#article-content");

  const edit = async () => {
    const url = "/api/article/" + ctx.articleSlug;
    const article = await fetch(url).then((r) => r.json());
    if (article.error != null) {
      alert(article.error);
      return;
    }

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
      h("section", {}, [
        h("button", { id: "save-button" }, "Save"),
        h("button", { id: "cancel-button" }, "Cancel"),
      ]),
    ]);

    editor.querySelector("#save-button").addEventListener("click", async () => {
      const titleInput = editor.querySelector("#article-title-field");
      const contentInput = editor.querySelector("#article-content-field");

      const response = await fetch(url, {
        method: "PATCH",
        body: JSON.stringify({
          title: titleInput.value,
          source: contentInput.value,
        }),
        headers: {
          "Content-Type": "application/json",
          authorization: "Bearer a", // TODO: Sort out authentication
        },
      }).then((r) => r.json());

      if (response.success) {
        window.location.reload();
      } else {
        alert(response.error);
      }
    });

    editor.querySelector("#cancel-button").addEventListener("click", () => {
      editor.remove();
      main.style.display = null;
    });

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
