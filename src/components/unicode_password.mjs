let css = `
input {
  width: 320px;
  height: 35px;
  padding-left: 5px;
  text-overflow: ellipsis;
  font-size: large;
  border-width: 2px;
  border-radius: 6px;
}
input:focus {
  border-width: 1px;
}
input:valid {
  border-color: #e5e4e9;
  box-shadow: inset -1px -1px 3px #ffffff, inset 1px 1px 3px #aaaaaa;
}
input:invalid,
input.invalid {
  border-color: #ffc0c0;
  border-width: 3px;
  box-shadow: 5px 5px 10px #ff5050, -5px -5px 10px #ffffff;
}`;
let html = `<input type="text" minlength="5" maxlength="255" required/>`;

const unicode_length = (s) => Array.from(s ?? '').length;

class UnicodePassword extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
  }

  connectedCallback() {
    let shadow = this.shadowRoot;
    shadow.innerHTML = `<style>${css}</style>${html}`;
    this.$input = shadow.querySelector('input');
    this.getAttributeNames().forEach(name => this.$input.setAttribute(name, this.getAttribute(name)));

    // cell can be filled with only one character
    shadow.addEventListener('input', (e) => {
      let t = e.target;
      unicode_length(t.value) < 5 ? t.classList.add('invalid') : t.classList.remove('invalid');
    });

    // dispatch custom event
    shadow.addEventListener('change', (e) => {
      this.dispatchEvent(new CustomEvent('onchange', { detail: e }));
    });
  }

  attributeChangedCallback(name, oldVal, newVal) {
    this.$input.setAttribute(name, newVal);
  }

  is_valid() {
    return unicode_length(this.$input.value) >= 5;
  }

  get value() {
    return this.$input.value;
  }
}
customElements.define('unicode-password', UnicodePassword)

