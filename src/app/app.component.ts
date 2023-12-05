import { Component, OnInit } from "@angular/core";
import { invoke } from "@tauri-apps/api/tauri";
import { t } from "@tauri-apps/api/tauri-5afe4a59";

@Component({
  selector: "app-root",
  templateUrl: "./app.component.html",
  styleUrls: ["./app.component.css"],
})
export class AppComponent implements OnInit {
  mode: string = "Normal";
  filename: string = new Date().toISOString() + ".md";
  ngOnInit() {
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

    // Command line focus
    let commandLine = document.getElementById("command-line")! as HTMLTextAreaElement;
    let normalMode = (this.mode === "Normal");
    document.addEventListener("keydown", (e) => {
      if (e.key === "i" && normalMode) {
        e.preventDefault();
        editor.focus();
        normalMode = false;
        this.mode = "Edit";
      }
      if (e.key === "Escape" && !normalMode) {
        commandLine.focus();
        normalMode = true;
        this.mode = "Normal";
      }
    });

    commandLine.addEventListener("focus", () => {
      normalMode = true;
      this.mode = "Normal";
    });
    editor.addEventListener("focus", () => {
      normalMode = false;
      this.mode = "Edit";
    });

    // command line
    // TODO: Change text color when valid command detected?
    commandLine.addEventListener("keydown", async (e) => {
      if (e.key === "Enter") {
        e.preventDefault();
        let command = commandLine.value;
        let contentNow = editor.value;
        let path = this.filename;
        commandLine.value = "";
        await invoke('execute', { command, contentNow, path });
      }
    });
  }
}

