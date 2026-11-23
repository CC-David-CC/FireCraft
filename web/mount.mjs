import {mountBinaryStudio} from './binary-studio.mjs';

export function mount(host) {
  return mountBinaryStudio(host, {
    name: 'FireCraft',
    extension: '.fw.png',
    openExtensions: ['.fw.png'],
    imports: ['.png', '.jpg', '.jpeg', '.gif', '.bmp', '.tif', '.tiff', '.webp'],
  });
}
