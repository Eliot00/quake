import { Component, h, State } from '@stencil/core';
import QuakeGen from '../../utils/quake-gen';

@Component({
  tag: 'quake-render',
  styleUrl: 'quake-render.css',
  shadow: true,
})
export class QuakeRender {
  @State() markdownData: any[] = [];

  componentWillLoad() {
    let content = `# [heading+](https://quake.inherd.org)
> blockquote

---

`;
    this.markdownData = new QuakeGen(content).gen();
  }

  render() {
    return <div>
      {this.markdownData.map((item: any) =>
        QuakeRender.conditionRender(item),
      )}
    </div>;
  }

  private static conditionRender(item: any) {
    let temp: string;
    switch (item.type) {
      case 'heading':
        temp = <h1 innerHTML={item.text} class='quake-heading' id={item.anchor} />;
        break;
      case 'blockquote':
        temp = <blockquote innerHTML={item.text} />;
        break;
      case 'hr':
        temp = <hr />;
        break;
      default:
        temp = <div>233</div>;
    }

    return temp;
  }
}