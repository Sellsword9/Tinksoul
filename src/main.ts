import { invoke } from "@tauri-apps/api/tauri";

// todo: Handle all ! assigments better

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
  // TODO: Send command to tauri to use match structure to run command if possible
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

// Save file part

const saveButton = document.getElementById('save-btn')! as HTMLButtonElement;

saveButton.addEventListener('click', () => {
  invoke('save_file', {
    content: editor.value,
  });
});

// Brain part
const brainInput = 
document.getElementById('brainpath')! as HTMLInputElement; // The value here is used at Saving
const brainButton = 
document.getElementById('brain-btn')! as HTMLButtonElement;
        
      // When focused, show button
      brainInput.addEventListener('focus', () => {
        brainButton.classList.remove("hiddenClass");
      });

      brainInput.addEventListener('keydown', (e: KeyboardEvent) => {
        if ((e.key === 'Enter') && brainInput.value != "") 
        {
          brainButton.click();
          e.preventDefault();
          e.stopPropagation();
        }
      });
      brainInput.addEventListener('blur', () => {
        brainButton.classList.add("hiddenClass");
      });


// Save file
const saveFileInput = document.getElementById('saveText')! as HTMLInputElement;
brainButton.addEventListener('click', () => {
  if (saveFileInput.value === "") {
    // FIXME: Show error somehow
  }else{
    invoke('save_on_brain', {
      filename: saveFileInput.value,
      brainpath: brainInput.value,
      content: editor.value,
    });
  }
});


// File setup
document.addEventListener('DOMContentLoaded', () => invoke('setup'));
