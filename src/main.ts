import { invoke } from "@tauri-apps/api/tauri";

let editor: HTMLTextAreaElement = document.getElementById("editor") as HTMLTextAreaElement;
let preview = document.getElementById("preview");

async function marked() {
  if (editor && preview) {
    preview.innerHTML = await invoke("markdownize", {
      md: editor.value,
    });
  }
}
editor.addEventListener("input", marked);

// Command line

const inputField = document.getElementById('command')! as HTMLInputElement;
const CommAnimationClass = 'CommAnimated';

function commandInput(command: String): void{
  inputField.value = "";
  //todo: Send command to tauri to use match structure to run command if possible
  console.log(command);
}

inputField.addEventListener('keydown', (e: KeyboardEvent) => {
  inputField.classList.remove(CommAnimationClass);
  if ((e.key === 'Enter') && inputField.value != "") {
    commandInput(inputField.value);
    e.preventDefault();
    e.stopPropagation();
  }

});
inputField.addEventListener('blur', () => {
  inputField.classList.add(CommAnimationClass);
  inputField.value = "";
});

// Todo?: Add a way to get the current directory and display it in the command line
