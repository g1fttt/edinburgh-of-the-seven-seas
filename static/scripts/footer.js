import { CustomElement } from "./shared.js";

class Footer extends CustomElement {
  constructor() {
    super()
  }

  async connectedCallback() {
    this.init("footer.html", "footer.css")
  }
}

customElements.define("footer-template", Footer)
