const ENV = Cypress.env();

describe("Access Pages", () => {
  it("navigate all pages", () => {
    cy.visit(`${ENV.HOST}`);

    cy.get("#list-input input").type("new todo");
    cy.get("#list-input button").click();

    cy.contains("new todo").should("exist");

    cy.wait(2000);

    cy.contains("new todo")
      .closest(".list-item")
      .within(($listItem) => {
        cy.get(".remove").click();
      });

    cy.wait(1000);

    cy.contains("new todo").should("not.exist");
  });
});
