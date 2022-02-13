import {Component, Host, h, Prop} from '@stencil/core';
import PadLeft from "../utils/PadLeft";
import DateFormat from "../utils/DateFormat";

@Component({
  tag: 'entry-card',
  styleUrl: 'entry-card.css',
  shadow: true,
})
export class EntryCard {
  @Prop() item: any;
  @Prop() type: string;
  @Prop() fileProp: number;

  showEntry(_id: string, _entry: string) {

  }

  editEntry(_id: string, _entry: string) {

  }

  showPdf(_id: string, _entry: string) {

  }

  render() {
    return (
      <Host>
        <div class="entry-show-list">
          <ion-card>
            <ion-card-header>
              <ion-card-subtitle># {PadLeft(this.item.id, 4, '')}
                {this.fileProp &&
                  <ion-icon name="document-outline" onClick={() => this.showPdf(this.type, this.item[this.fileProp])}/>
                }
                <ion-icon name="book-outline" onClick={() => this.showEntry(this.item.id, this.type)}/>
                <ion-icon name="create-outline" onClick={() => this.editEntry(this.item.id, this.type)}/>
              </ion-card-subtitle>
              <ion-card-title>{this.item.title}</ion-card-title>
            </ion-card-header>
            <ion-card-content>
              {this.item.description && <p>{this.item.description}</p>}
              <ion-badge slot="start">{DateFormat(this.item.created_date)}</ion-badge>
            </ion-card-content>
          </ion-card>
        </div>
      </Host>
    );
  }

}
