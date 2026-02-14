# Bithumb-Api - Overview

**Pages:** 4

---

## 자주 묻는 질문 (FAQ)

**URL:** https://apidocs.bithumb.com/docs/%EC%9E%90%EC%A3%BC-%EB%AC%BB%EB%8A%94-%EC%A7%88%EB%AC%B8-faq

**Contents:**
- [API 2.0]빗썸 API 이용 안내
- 자주 묻는 질문 (FAQ)
- 출금 API를 이용하는데 invalid Parameter 에러가 확인됩니다
- API 호출 시 IP가 차단되었다고 확인됩니다

‘특정 금융거래 정보의 보고 및 이용에 관한 법률’에 따라 2022년 3월 25일부터 빗썸 외 지갑으로 가상자산을 출금하는 경우, 가상자산 송/수신인 정보를 제공할 수 있는 VASP로의 이전만을 허용하게 되었습니다.

출금 API를 통해 빗썸 외 지갑으로 출금을 진행하시는 경우, 아래 항목을 반드시 확인 해주세요.

◾ 개인 지갑으로 출금 시 필수 요청 변수

◾ 법인 지갑으로 출금 시 필수 요청 변수

invalid Parameter 외에 ‘다시 시도’ 해달라는 에러가 확인되는 경우, 입력하신 수취인 정보와 실제 수취인 정보가 다를 수 있으니 수취인 정보를 재확인 해주시기 바랍니다.

빗썸 Open API는 안정적인 서비스 제공을 위해 API 별 요청 수를 제한합니다. 제한된 요청 수 이상으로 초과 요청하시는 경우 IP가 일시적으로 차단되오니 이용에 주의하시기 바랍니다.

제한 시간이 경과하면 차단이 자동 해제되며, 실패 호출도 제한에 포함되오니 참고 부탁드립니다.

Updated about 1 month ago

---

## 인증 헤더 만들기

**URL:** https://apidocs.bithumb.com/docs/인증-헤더-만들기

**Contents:**
- [API 2.0]빗썸 API 이용 안내
- 인증 헤더 만들기
- REST API 요청 포맷
- JWT 인증 토큰 만들기
  - (예시) 파라미터가 없을 경우
  - (예시) 파라미터가 있는 경우

REST API는 HTTP를 통해 호출이 이루어집니다.

POST, PUT, DELETE 요청에 body가 존재하는 경우 JSON 형식으로 파라미터를 전송해야 합니다. 유효한 컨텐츠 타입의 예시는 다음과 같으며, 각 프로그래밍 언어 라이브러리에 따라 약간의 차이가 있을 수 있습니다.

REST API 요청시, 발급받은 API Key와 Secret Key로 토큰을 생성하여 Authorization 헤더를 통해 전송합니다. 토큰은 JWT(https://jwt.io) 형식을 따릅니다.

서명 방식은 HS256 을 권장하며, 서명에 사용할 Secret은 발급받은 Secret Key를 사용합니다. 페이로드의 구성은 다음과 같습니다.

Private API 요청 시 발급받은 API Key와 Secret Key를 이용하여 5개의 파라미터를 헤더에 추가하여 전송합니다.

현재의 시간을 밀리초 (millisecond, ms)로 표현한 값 (예 : 1655280216476)

해싱된 query string (파라미터가 있을 경우 필수)

query_hash를 생성하는 데에 사용한 알고리즘 (기본값 : SHA512)

생성된 인증 헤더의 페이로드 예시입니다.

HTTP 쿼리 문자열, 혹은 body를 통해 파라미터를 전달하는 경우 모두 JWT 페이로드의 query_hash 값을 설정해야합니다.

파라미터의 자료형 중 배열이 존재하는 경우, 올바른 query string의 형태는 다음과 같습니다.

key[]=value1&key[]=value2 ...

이와 다른 형태로 요청하면 토큰 검증에 실패할 수 있으니 유의하시기 바랍니다.

Updated about 1 month ago

**Examples:**

Example 1 (unknown):
```unknown
Content-Type: application/json; charset=utf-8
```

Example 2 (json):
```json
{
  "access_key": "L7rVaYfBIc2BDsnlQGfkR93d6DoOAJCw7mJr5Eso",
  "nonce": "6f5570df-d8bc-4daf-85b4-976733feb624",
  "timestamp": 1712230310689,
  "query_hash": "1c2362ca9d79947582cae192acf63efb8756caa49af1eb64b12ba45617165431a3dd3e47d0476bc2a347b7a1ea512db7f316f56144084b1493166e3c9113a8eb",
  "query_hash_alg": "SHA512"
}
```

Example 3 (javascript):
```javascript
const jwt = require('jsonwebtoken')
const { v4: uuidv4 } = require('uuid')

const accessKey = '발급받은 API KEY'
const secretKey = '발급받은 SECRET KEY'

const payload = {
    access_key: accessKey,
    nonce: uuidv4(),
    timestamp: Date.now()
};

console.log(payload);

const jwtToken = jwt.sign(payload, secretKey)
const authorizationToken = `Bearer ${jwtToken}`

console.log(authorizationToken);
```

Example 4 (swift):
```swift
# Python 3
# pip3 installl pyJwt
import jwt 
import uuid
import time

accessKey = "발급받은 API KEY"
secretKey = "발급받은 SECRET KEY"

payload = {
    'access_key': accessKey,
    'nonce': str(uuid.uuid4()),
    'timestamp': round(time.time() * 1000)
}

print(payload, "\n")
    
jwt_token = jwt.encode(payload, secretKey)
authorization_token = 'Bearer {}'.format(jwt_token)

print(authorization_token)
```

---

## 자주 묻는 질문 (FAQ)

**URL:** https://apidocs.bithumb.com/docs/자주-묻는-질문-faq

**Contents:**
- [API 2.0]빗썸 API 이용 안내
- 자주 묻는 질문 (FAQ)
- 출금 API를 이용하는데 invalid Parameter 에러가 확인됩니다
- API 호출 시 IP가 차단되었다고 확인됩니다

‘특정 금융거래 정보의 보고 및 이용에 관한 법률’에 따라 2022년 3월 25일부터 빗썸 외 지갑으로 가상자산을 출금하는 경우, 가상자산 송/수신인 정보를 제공할 수 있는 VASP로의 이전만을 허용하게 되었습니다.

출금 API를 통해 빗썸 외 지갑으로 출금을 진행하시는 경우, 아래 항목을 반드시 확인 해주세요.

◾ 개인 지갑으로 출금 시 필수 요청 변수

◾ 법인 지갑으로 출금 시 필수 요청 변수

invalid Parameter 외에 ‘다시 시도’ 해달라는 에러가 확인되는 경우, 입력하신 수취인 정보와 실제 수취인 정보가 다를 수 있으니 수취인 정보를 재확인 해주시기 바랍니다.

빗썸 Open API는 안정적인 서비스 제공을 위해 API 별 요청 수를 제한합니다. 제한된 요청 수 이상으로 초과 요청하시는 경우 IP가 일시적으로 차단되오니 이용에 주의하시기 바랍니다.

제한 시간이 경과하면 차단이 자동 해제되며, 실패 호출도 제한에 포함되오니 참고 부탁드립니다.

Updated about 1 month ago

---

## API 주요 에러 코드

**URL:** https://apidocs.bithumb.com/docs/api-주요-에러-코드

**Contents:**
- [API 2.0]빗썸 API 이용 안내
- API 주요 에러 코드
- 개요
- 400 Bad Request
- 401 Unauthorized
- 403 Forbidden
- 404 Not Found
- 500 Internal Server Error

API 요청값이 유효하지 않거나 처리 중 오류가 발생한 경우, HTTP 상태 코드와 함께 다음과 같은 형태의 JSON body가 리턴됩니다.

401 Unauthorized 오류는 대부분 JWT 서명이 올바르게 되지 않았을 때 발생합니다. 인증 헤더 만들기 문서를 참조하시어 서명이 올바르게 되었는지 확인해주세요.

403 Forbidden 오류는 대부분 접근 권한이 없거나, 운영 정책에 따라 제한된 기능일 수 있습니다. 더 궁금하신 점은 고객센터로 문의해주세요.

500 Internal Server Error는 요청에는 문제가 없으나, 서버에서 데이터를 처리하는 과정에서 일시적인 이슈(예: 응답 지연)가 발생했을 때 나타납니다. 잠시 후 다시 시도해주시기 바랍니다.

Updated about 1 month ago

**Examples:**

Example 1 (json):
```json
{
  "error": {
    "message": "오류에 대한 설명",
    "name": "오류 코드"
  }
}
```

---
