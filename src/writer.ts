export interface Writer {
    write(text: string): void;
    clear(): void;
}

export class HTMLWriter implements Writer {

    constructor(private element: HTMLElement) { }

    write(text: string) {
        this.element.innerText += text
    }

    clear(): void {
        this.element.innerText = '';
    }

}