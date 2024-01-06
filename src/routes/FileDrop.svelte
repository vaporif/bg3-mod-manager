<script lang="ts">
  import { event } from '@tauri-apps/api'
  import { onDestroy } from 'svelte'

  export let extensions: null | string[] = null
  export let handleFiles: (files: string[]) => void = () => {
  }

  function getValidPaths(paths: string[]) {
    if (extensions === null) {
      alert(paths);
      return paths
    }
    let validPaths = []
    for (const path of paths) {
      for (const ext of extensions) {
        if (path.endsWith('.' + ext)) {
          validPaths.push(path)
          break
        }
      }
    }
    return validPaths
  };

  let files: string[] = []

  const fileDropHover = event.listen('tauri://file-drop-hover', (e) => {
    files = getValidPaths(e.payload as string[])
    console.log(files);
  });
  
  onDestroy(async () => {
    const unlisten = await fileDropHover
    unlisten()
  });

  const fileDrop = event.listen('tauri://file-drop', (e) => {
    const payload = e.payload as string[]
    files = getValidPaths(payload)
    if (files.length > 0) {
      handleFiles(files)
    }
    files = []
  });

  onDestroy(async () => {
    const unlisten = await fileDrop
    unlisten()
  });

  const fileDropCancelled = event.listen('tauri://file-drop-cancelled', () => {
    files = []
  });

  onDestroy(async () => {
    const unlisten = await fileDropCancelled
    unlisten()
  });
</script>

<slot/>
