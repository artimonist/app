let css = `@scope {
:scope {
  width: 320px;
  height: 35px;
  padding: 0px 5px 0px 5px;
  text-overflow: ellipsis;
  font-size: large;
  border-width: 2px;
  border-radius: 6px;
}
:scope:valid {
  border-color: #e5e4e9;
  box-shadow: inset -1px -1px 3px #ffffff, inset 1px 1px 3px #aaaaaa;
}
:scope:invalid{
  border-color: #ffc0c0;
  border-width: 3px;
  box-shadow: 5px 5px 10px #ff5050, -5px -5px 10px #ffffff;
}}`;

const unicode_length = (s) => Array.from(s ?? '').length;

class UnicodeInput extends HTMLInputElement {
  constructor() {
    super();
    this.type = 'text';
  }

  connectedCallback() {
    this.innerHTML = `<style>${css}</style>`;
  }

  checkValidity() {
    return unicode_length(this.value) >= this.minlength && unicode_length(this.value) <= this.maxlength;
  }
}
customElements.define('unicode-input', UnicodeInput, { extends: 'input' });
