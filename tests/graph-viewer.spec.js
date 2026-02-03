// @ts-check
const { test, expect } = require('@playwright/test');

/**
 * Helper function to check if a canvas has been drawn on (is not blank/white)
 * @param {import('@playwright/test').Page} page - The Playwright page object
 * @returns {Promise<boolean>} - True if canvas has content, false if blank
 */
async function canvasHasContent(page) {
  return await page.evaluate(() => {
    const canvas = document.getElementById('graph');
    const ctx = canvas.getContext('2d');
    const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
    
    // Check if any pixel is not white/blank
    // Using 250 as threshold instead of 255 to account for canvas background color (#fafafa = 250,250,250)
    for (let i = 0; i < imageData.data.length; i += 4) {
      if (imageData.data[i] !== 250 || imageData.data[i+1] !== 250 || imageData.data[i+2] !== 250) {
        return true;
      }
    }
    return false;
  });
}

test.describe('Graph Viewer', () => {
  test('should display page and load graph data', async ({ page }) => {
    await page.goto('/');

    // Check page title
    await expect(page).toHaveTitle('Salesforce Types Explorer');

    // Check header is visible
    await expect(page.locator('h1')).toContainText('Salesforce Types Explorer');

    // Check statistics are loaded
    await expect(page.locator('#totalTypes')).toContainText('2,682');
    await expect(page.locator('#totalEdges')).toContainText('4,906');
    await expect(page.locator('#totalCategories')).toContainText('83');
  });

  test('should display initial overview graph on page load', async ({ page }) => {
    await page.goto('/');

    // Wait for graph to be drawn by checking for the graph note
    await page.waitForSelector('.graph-note', { state: 'visible' });

    // Check that graph note mentions overview
    const graphNote = page.locator('.graph-note');
    await expect(graphNote).toContainText('Overview of top categories');

    // Check canvas exists and has content
    const canvas = page.locator('#graph');
    await expect(canvas).toBeVisible();

    // Verify canvas has been drawn on by checking it's not blank
    const hasContent = await canvasHasContent(page);
    expect(hasContent).toBe(true);
  });

  test('should display type details when clicking a category', async ({ page }) => {
    await page.goto('/');

    // Click on objects category
    await page.getByText('objects (39)').click();

    // Check that category details are shown
    await expect(page.locator('h2')).toContainText('Category: objects');
    await expect(page.locator('text=39 types in this category')).toBeVisible();

    // Check that some types are listed
    await expect(page.locator('text=CustomObject')).toBeVisible();
    await expect(page.locator('text=CustomField')).toBeVisible();
  });

  test('should display type-specific graph when clicking a type', async ({ page }) => {
    await page.goto('/');

    // Click on objects category first
    await page.getByText('objects (39)').click();
    
    // Wait for category header to appear
    await page.waitForSelector('h2:has-text("Category: objects")', { state: 'visible' });
    
    // Then click on CustomObject type
    await page.getByText('CustomObject', { exact: true }).click();

    // Wait for type details heading to appear
    await page.waitForSelector('h2:has-text("CustomObject")', { state: 'visible' });

    // Check type details are displayed
    await expect(page.locator('h2')).toContainText('CustomObject');
    await expect(page.locator('text=Category: objects')).toBeVisible();
    await expect(page.locator('text=Dependencies: 28')).toBeVisible();

    // Check dependencies are listed
    await expect(page.locator('h3')).toContainText('Contains Dependencies');
    await expect(page.locator('text=CustomField')).toBeVisible();

    // Verify canvas has been redrawn with type-specific graph
    const hasContent = await canvasHasContent(page);
    expect(hasContent).toBe(true);
  });

  test('should filter graph by relationship types', async ({ page }) => {
    await page.goto('/');

    // Navigate to a type with dependencies
    await page.getByText('objects (39)').click();
    await page.waitForSelector('h2:has-text("Category: objects")', { state: 'visible' });
    await page.getByText('CustomObject', { exact: true }).click();
    await page.waitForSelector('h2:has-text("CustomObject")', { state: 'visible' });

    // Uncheck "Contains" filter
    await page.locator('#filterContains').uncheck();

    // Check that filter is unchecked
    await expect(page.locator('#filterContains')).not.toBeChecked();

    // Re-check the filter
    await page.locator('#filterContains').check();
    await expect(page.locator('#filterContains')).toBeChecked();
  });

  test('should search for types', async ({ page }) => {
    await page.goto('/');

    // Type in search box
    await page.locator('#searchInput').fill('CustomObject');
    
    // Wait for search results to appear
    await page.waitForSelector('h2:has-text("Search Results")', { state: 'visible' });

    // Check search results are displayed
    await expect(page.locator('h2')).toContainText('Search Results');
    await expect(page.locator('text=CustomObject')).toBeVisible();

    // Click on a search result
    await page.getByText('CustomObject', { exact: true }).first().click();
    
    // Wait for type details to appear
    await page.waitForSelector('h2:has-text("CustomObject")', { state: 'visible' });

    // Verify type details are shown
    await expect(page.locator('h2')).toContainText('CustomObject');
  });

  test('should clear search when clicking categories', async ({ page }) => {
    await page.goto('/');

    // Search for something
    await page.locator('#searchInput').fill('Flow');
    
    // Wait for search results
    await page.waitForSelector('h2:has-text("Search Results")', { state: 'visible' });

    // Click on a category
    await page.getByText('flows (51)').click();

    // Check that search was cleared
    const searchValue = await page.locator('#searchInput').inputValue();
    expect(searchValue).toBe('');

    // Check category view is shown
    await expect(page.locator('h2')).toContainText('Category: flows');
  });

  test('should navigate between types via dependencies', async ({ page }) => {
    await page.goto('/');

    // Navigate to CustomObject
    await page.getByText('objects (39)').click();
    await page.waitForSelector('h2:has-text("Category: objects")', { state: 'visible' });
    await page.getByText('CustomObject', { exact: true }).click();
    await page.waitForSelector('h2:has-text("CustomObject")', { state: 'visible' });

    // Click on a dependency (CustomField)
    await page.getByText('CustomField').first().click();
    
    // Wait for CustomField details to appear
    await page.waitForSelector('h2:has-text("CustomField")', { state: 'visible' });

    // Verify we navigated to CustomField
    await expect(page.locator('h2')).toContainText('CustomField');
  });

  test('should handle types with no dependencies', async ({ page }) => {
    await page.goto('/');

    // Search for a type that likely has no dependencies
    await page.locator('#searchInput').fill('Gender');
    
    // Wait for search results
    await page.waitForSelector('h2:has-text("Search Results")', { state: 'visible' });

    // Click on it
    await page.getByText('Gender', { exact: true }).first().click();
    
    // Wait for type details to appear
    await page.waitForSelector('h2:has-text("Gender")', { state: 'visible' });

    // Should show type details even with no dependencies
    await expect(page.locator('h2')).toContainText('Gender');

    // Graph should show message about no dependencies or be drawn
    const graphHasContent = await canvasHasContent(page);
    // Graph might show "No dependencies" text
    expect(graphHasContent).toBeTruthy();
  });

  test('should show all filter checkboxes as checked by default', async ({ page }) => {
    await page.goto('/');

    // Check all filters are checked
    await expect(page.locator('#filterContains')).toBeChecked();
    await expect(page.locator('#filterExtends')).toBeChecked();
    await expect(page.locator('#filterGeneric')).toBeChecked();
  });

  test('should display welcome message initially', async ({ page }) => {
    await page.goto('/');

    // Check welcome message is displayed
    await expect(page.locator('h2')).toContainText('Welcome to Salesforce Types Explorer');
    await expect(page.locator('text=About')).toBeVisible();
    await expect(page.locator('text=Contains:')).toBeVisible();
    await expect(page.locator('text=Extends:')).toBeVisible();
    await expect(page.locator('text=Generic:')).toBeVisible();
  });
});
