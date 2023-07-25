const ENV = Cypress.env();

describe("Access Pages", () => {
  it("navigate all pages", () => {
    cy.visit(`${ENV.HOST}`);

    cy.get("#list-input input").type("new todo");
    cy.get("#list-input button").click();

    cy.contains("new todo");
  });
});
