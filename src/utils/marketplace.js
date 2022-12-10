import { v4 as uuid4 } from "uuid";
import { parseNearAmount } from "near-api-js/lib/utils/format";

const GAS = 100000000000000;

export function createProduct(product) {
    product.id = uuid4();
    product.price = parseNearAmount(product.price + "");
    return window.contract.set_product({ product });
}

export function getProducts() {
    return window.contract.get_products();
}

export async function buyProduct({ id, price }) {
    await window.contract.buy_product({ productId: id }, GAS, price);
}