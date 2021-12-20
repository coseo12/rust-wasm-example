import init, { ImageProccessor } from './pkg/mld_demo.js';

const canvas = document.getElementById('canvas');
const button = document.getElementById('process');

(async () => {
  await init();
  let img = Object.assign(new Image(), {
    src: 'https://images.unsplash.com/photo-1567540227188-f27fb2e2babd?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1035&q=80',
    crossOrigin: 'anonymous',
  });

  await img.decode('utf8');

  let { width, height } = img;
  Object.assign(canvas, { width, height });
  canvas.getContext('2d').drawImage(img, 0, 0);

  const processor = new ImageProccessor(canvas);

  button.addEventListener('click', () => {
    processor.hue_roteate();
    processor.redraw();
  });
})();
