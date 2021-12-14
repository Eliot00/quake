import {newSpecPage} from '@stencil/core/testing';
import {QuakeRender} from './quake-render';

describe('quake-render', () => {
  it('render heading', async () => {
    const {root} = await newSpecPage({
      components: [QuakeRender],
      html: '<quake-render content="# h1 \n "></quake-render>',
    });
    expect(root).toEqualHtml(`
      <quake-render content="# h1 \n ">
        <mock:shadow-root>
          <div>
            <h1 class="quake-heading" id="h1">
               h1
            </h1>
          </div>
        </mock:shadow-root>
      </quake-render>
    `);
  });
  it('render list', async () => {
    const {root} = await newSpecPage({
      components: [QuakeRender],
      html: '<quake-render content="- item 1 \n     - sub \n - item 2"></quake-render>',
    });

    expect(root).toEqualHtml(`
      <quake-render content="- item 1 \n     - sub \n - item 2">
        <mock:shadow-root>
          <div>
            <ul>
              <li>
                <span>
                  item 1
                </span>
                <ul>
                  <li>
                    <span>
                      sub
                    </span>
                  </li>
                </ul>
              </li>
              <li>
                <span>
                  item 2
                </span>
              </li>
            </ul>
          </div>
        </mock:shadow-root>
      </quake-render>
    `);
  });

  it('renders with values', async () => {

  });
});
