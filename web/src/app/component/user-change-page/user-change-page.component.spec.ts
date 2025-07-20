import { ComponentFixture, TestBed } from '@angular/core/testing';

import { UserChangePageComponent } from './user-change-page.component';

describe('UserChangePageComponent', () => {
  let component: UserChangePageComponent;
  let fixture: ComponentFixture<UserChangePageComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [UserChangePageComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(UserChangePageComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
