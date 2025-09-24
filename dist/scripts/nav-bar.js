class NavBar extends HTMLElement {
  template;

  constructor() {
    super()

    this.template = document.createElement('template')
  }

  async connectedCallback() {
    const shadowRoot = this.attachShadow({ mode: 'closed' })

    const htmlResp = await fetch('templates/nav-bar.html')
    this.template.innerHTML = await htmlResp.text()

    shadowRoot.appendChild(this.template.content)
    shadowRoot.querySelectorAll(".nav-bar-elem")
      .forEach(link => this.markIfActive(link))
  }

  markIfActive(link) {
    const href = link.getAttribute("href")

    const isCurrentLinkActive = href === window.location.pathname
    const isRoot = window.location.pathname === "/"

    if (isCurrentLinkActive || (isRoot && href === "/index.html")) {
      link.classList.add("active")
    }
  }
}

customElements.define('nav-bar', NavBar)
