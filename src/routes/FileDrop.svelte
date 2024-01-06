<script lang="ts">
  import { event } from '@tauri-apps/api'
  import { onDestroy } from 'svelte'

  export let extensions: null | string[] = null
  export let onDrop: (files: string[]) => void = () => {
  }


  export let onHover: (files: string[]) => void = () => {
  }

  function getValidPaths(paths: string[]) {
    if (extensions === null) {
      return paths;
    }
    let validPaths = []
    for (const path of paths) {
      for (const ext of extensions) {
        if (path.endsWith('.' + ext)) {
          validPaths.push(path);
          break;
        }
      }
    }
    return validPaths;
  };

  const fileDropHover = event.listen('tauri://file-drop-hover', (e) => {
    let files = getValidPaths(e.payload as string[]);
    onHover(files);
  });
  
  onDestroy(async () => {
    const unlisten = await fileDropHover
    unlisten()
  });

  const fileDrop = event.listen('tauri://file-drop', (e) => {
    const payload = e.payload as string[]
    let files = getValidPaths(payload)
    if (files.length > 0) {
      onDrop(files)
    }
  });

  onDestroy(async () => {
    const unlisten = await fileDrop
    unlisten()
  });

</script>

<slot/>
