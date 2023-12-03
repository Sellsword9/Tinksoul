import { Component, OnInit } from "@angular/core";
import { invoke } from "@tauri-apps/api/tauri";

@Component({
  selector: "app-root",
  templateUrl: "./app.component.html",
  styleUrls: ["./app.component.css"],
})
export class AppComponent implements OnInit {
  ngOnInit() {
    // import { save } from "@tauri-apps/api/dialog"

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
    // File setup
    document.addEventListener('DOMContentLoaded', () => invoke('setup'));
  }
}
