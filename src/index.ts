import Interpreter from "./interpreter"
import { HTMLWriter } from "./writer";

function init() {
    const inputElement = document.getElementById('input') as HTMLTextAreaElement
    const outputElement = document.getElementById('output')

    if (!outputElement) {
        // TODO: implement this properly
        return;
    }

    const writer = new HTMLWriter(outputElement)
    const interpreter = new Interpreter(writer)
    document.getElementById('run')?.addEventListener('click', () => {
        interpreter.run(inputElement.value)
    })
}

document.addEventListener('DOMContentLoaded', init)
