import { Component } from '@angular/core';
import { invoke } from '@tauri-apps/api/tauri';

interface FileInfo {
  name: string;
  size: number;
  path: string;
}

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css'],
})
export class AppComponent {
  public largestFileList: FileInfo[] = [];
  public loading: boolean = false;

  public analyzeDir(event: SubmitEvent, name: string): void {
    event.preventDefault();

    this.largestFileList = [];
    this.loading = true;
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    invoke<string>('analyze_dir', { name }).then((FileInfo) => {
      this.largestFileList = JSON.parse(FileInfo);
      this.loading = false;
    });
  }
}
