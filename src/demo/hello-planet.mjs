export class HelloPlanet extends HTMLElement {
  constructor() {
    super();
  }
  connectedCallback() {
    // Attach a shadow root to the element.
    let shadowRoot = this.attachShadow({ mode: 'open' });
    let planet = this.getAttribute('planet')
    shadowRoot.innerHTML = `<p>hello ${planet}</p>`;
  }
}