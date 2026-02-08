describe('App Launch', () => {
  it('should have correct title', async () => {
    const title = await browser.getTitle();
    expect(title).toBe('Multi-VM Workspace');
  });

  it('should display workset list view', async () => {
    const listView = await browser.$('#view-list');
    await expect(listView).toExist();
  });

  it('should have create workset button', async () => {
    const btn = await browser.$('#btn-new-workset');
    await expect(btn).toExist();
    await expect(btn).toBeDisplayed();
  });

  it('should open create form on button click', async () => {
    const btn = await browser.$('#btn-new-workset');
    await btn.click();

    const form = await browser.$('#view-create');
    await expect(form).toBeDisplayed();
  });

  it('should have theme toggle', async () => {
    const toggle = await browser.$('#btn-theme-toggle');
    await expect(toggle).toExist();
  });
});
