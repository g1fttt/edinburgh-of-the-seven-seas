export class CustomElement extends HTMLElement {
  constructor() {
    super()
  }

  async init(templateFilename, templateStyleFilename) {
    const htmlResp = await fetch(`templates/${templateFilename}`)

    const baseCssResp = await fetch("/styles/base.css")
    const cssResp = await fetch(`/styles/${templateStyleFilename}`)

    let template = document.createElement("template")
    template.innerHTML = `
      <style>${await cssResp.text()}</style>
      ${await htmlResp.text()}
    `

    const baseCssSheet = new CSSStyleSheet()
    await baseCssSheet.replace(await baseCssResp.text())

    const shadowRoot = this.attachShadow({ mode: "closed" })
    shadowRoot.adoptedStyleSheets = [baseCssSheet]
    shadowRoot.appendChild(template.content)

    return shadowRoot;
  }
}
